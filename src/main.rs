#![feature(process_exitcode_placeholder)]
mod ast;
mod error;
mod eval;
mod lex;
mod parser;

use error::Error;
use eval::{Context, Value};
use image::{Rgb, RgbImage};
use num_complex::Complex;
use std::cmp;
use std::env;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;

const ITERATIONS: u32 = 200;

#[derive(StructOpt)]
struct Args {
    init: Expr,
    iter: Expr,
    width: NonZeroU32,
    height: NonZeroU32,
    output_path: PathBuf,
}

struct Expr(ast::Expr);

impl FromStr for Expr {
    type Err = Error;

    fn from_str(data: &str) -> Result<Expr, Error> {
        lex::tokenize(data)
            .and_then(|tokens| parser::parse(&tokens))
            .map(Expr)
    }
}

fn abs(num: Complex<f64>) -> f64 {
    (num.re * num.re + num.im * num.im).sqrt()
}

fn work(
    init: &ast::Expr,
    iter: &ast::Expr,
    start: u32,
    length: u32,
    width: u32,
    height: u32,
) -> Result<Vec<Rgb<u8>>, Error> {
    let mut context = Context::new();
    context.set("WIDTH", width as f64);
    context.set("HEIGHT", height as f64);
    context.set("X", 0.0);
    context.set("Y", 0.0);
    context.set("Z", 0.0);

    let mut pixels = Vec::with_capacity(length as usize);
    for (x, y) in (start..start + length).map(|n| (n % width, n / height)) {
        context.update("X", x as f64);
        context.update("Y", y as f64);

        let mut z = match context.eval(&init)? {
            Value::Complex(z) => z,
            _ => {
                return Err(Error {
                    message: "Init did not evaluate to a complex number",
                    position: init.position,
                });
            }
        };

        let mut pixel = Rgb([0, 0, 0]);
        for i in 0..ITERATIONS {
            context.update("Z", z);

            z = match context.eval(&iter)? {
                Value::Complex(z) => z,
                _ => {
                    return Err(Error {
                        message: "Iter did not evaluate to a complex number",
                        position: init.position,
                    });
                }
            };

            if abs(z) >= 2.0 {
                let pv = (i as f64 / ITERATIONS as f64 * 255.0) as u8;

                pixel = Rgb([pv, 0, 0]);
                break;
            }
        }

        pixels.push(pixel);
    }

    Ok(pixels)
}

fn render(init: ast::Expr, iter: ast::Expr, width: u32, height: u32) -> Result<RgbImage, Error> {
    let total = width * height;
    let length = total / num_cpus::get() as u32;

    let pixels = if length != 0 {
        let exprs = Arc::new((init, iter));
        let mut threads = Vec::new();
        let mut position = 0;
        while position < total {
            let length = cmp::min(total - position, length);
            let exprs = exprs.clone();

            threads.push(thread::spawn(move || {
                work(&exprs.0, &exprs.1, position, length, width, height)
            }));

            position += length;
        }

        threads
            .into_iter()
            .try_fold(Vec::new(), |mut acc, thread| {
                acc.extend(thread.join().unwrap()?);
                Ok(acc)
            })?
    } else {
        work(&init, &iter, 0, total, width, height)?
    };

    Ok(RgbImage::from_fn(width, height, |x, y| {
        pixels[(y * width + x) as usize]
    }))
}

fn main() -> ExitCode {
    let args = match Args::from_iter_safe(env::args()) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    let image = match render(
        args.init.0,
        args.iter.0,
        args.width.into(),
        args.height.into(),
    ) {
        Ok(image) => image,
        Err(err) => {
            println!("Error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = image.save(&args.output_path) {
        println!("Error saving image: {}", err);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
