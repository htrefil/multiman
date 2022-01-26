# multiman
A parallel Mandelbrot and Julia set-style fractal renderer supporting custom user expressions.
It leverages automatic differentiation and a distance estimate to make the details less noisy.
All available CPU cores are used to render images in parallel.

## Usage
```
multiman 0.2.0

USAGE:
    multiman <init> <first> <iter> <width> <height> <output-path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <init>           Initialization expression (the value of the pixel)
    <first>          Start of the iteration
    <iter>           Iteration expression
    <width>          Width of the image
    <height>         Height of the image
    <output-path>    Path of the resulting image
```

In the expressions, the following variables are available:
```
w    The width of the image
h    The height of the image
x    The current x coordinate
y    The current y coordinate
c    The initial complex value of the current pixel
z    The value of the previous iteration
```
They can be combined using arithmetic operations.

## Examples
```
multiman "(x / w * 2 - 1) + (y / h * 2 - 1.0) * i" "c" "z * z + (0.1 + 0.65i)" 1000 1000 examples/1.png
```
![1](examples/1.png)

```
multiman "x * 2 / w - 1.5 + (y * 2 / h - 1) * i" "0" "z * z + c" 1000 1000 examples/2.png
```
![2](examples/2.png)
