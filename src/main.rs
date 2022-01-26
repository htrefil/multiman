mod ast;
mod error;
mod eval;
mod lex;
mod parser;

use error::Error;
use eval::Context;
use image::{Rgb, RgbImage};
use num_complex::Complex;
use std::cmp;
use std::env;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use structopt::StructOpt;

const ITERATIONS: u32 = 200;

#[derive(StructOpt)]
struct Args {
    #[structopt(help = "Initialization expression (the value of the pixel)")]
    init: Expr,
    #[structopt(help = "Start of the iteration")]
    first: Expr,
    #[structopt(help = "Iteration expression")]
    iter: Expr,
    #[structopt(help = "Width of the image")]
    width: NonZeroU32,
    #[structopt(help = "Height of the image")]
    height: NonZeroU32,
    #[structopt(help = "Path of the resulting image")]
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
    first: &ast::Expr,
    iter: &ast::Expr,
    start: u32,
    length: u32,
    width: u32,
    height: u32,
) -> Result<Vec<Rgb<u8>>, Error> {
    let mut context = Context::new();
    context.width = width as f64;
    context.height = height as f64;

    let mut pixels = Vec::with_capacity(length as usize);
    for (x, y) in (start..start + length).map(|n| (n % width, n / height)) {
        context.x = x as f64;
        context.y = y as f64;

        let mut init = context.eval(&init)?;
        init.d = 1.0.into();
        context.c = init;

        let mut z = context.eval(&first)?;
        for _ in 0..ITERATIONS {
            context.z = z;

            z = context.eval(&iter)?;

            if abs(z.v) >= 2.0 {
                break;
            }
        }

        let (r, dr) = (abs(z.v), abs(z.d));
        let distance = width as f64 * 0.7 * f64::ln(r) * r / dr;
        let pv = std::cmp::min(f64::floor(255.0 * distance) as u8, 255);
        pixels.push(Rgb([pv, pv, pv]));
    }

    Ok(pixels)
}

fn render(
    init: ast::Expr,
    first: ast::Expr,
    iter: ast::Expr,
    width: u32,
    height: u32,
) -> Result<RgbImage, Error> {
    let total = width * height;
    let length = total / num_cpus::get() as u32;

    let pixels = if length != 0 {
        let exprs = Arc::new((init, first, iter));
        let mut threads = Vec::new();
        let mut position = 0;
        while position < total {
            let length = cmp::min(total - position, length);
            let exprs = exprs.clone();

            threads.push(thread::spawn(move || {
                work(
                    &exprs.0, &exprs.1, &exprs.2, position, length, width, height,
                )
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
        work(&init, &first, &iter, 0, total, width, height)?
    };

    Ok(RgbImage::from_fn(width, height, |x, y| {
        pixels[(y * width + x) as usize]
    }))
}

fn main() {
    let args = match Args::from_iter_safe(env::args()) {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let image = match render(
        args.init.0,
        args.first.0,
        args.iter.0,
        args.width.into(),
        args.height.into(),
    ) {
        Ok(image) => image,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    if let Err(err) = image.save(&args.output_path) {
        println!("Error saving image: {}", err);
        return;
    }
}
