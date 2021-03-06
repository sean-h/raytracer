# Ray Tracer

A Ray Tracer written in Rust based on the ray tracer from [Ray Tracing in One Weekend](https://github.com/petershirley/raytracinginoneweekend).

## Features

- Multithreaded rendering
- Scenes are loaded from .toml files
- Depth of Field
- Motion Blur

## Dependencies

- [rand](https://crates.io/crates/rand)
- [image](https://crates.io/crates/image)
- [toml](https://github.com/alexcrichton/toml-rs)
- [threadpool](https://crates.io/crates/threadpool)
- [tdmath](https://github.com/sean-h/tdmath)
- [cmdpro](https://github.com/sean-h/cmdpro)

## Usage

To render a sample scene run the following. The output will be named `output.png` and will be located in the project root directory.

```
cargo run --release -- --scene scenes/cornell.toml --width 400 --height 400 --samples 200
```

## Examples

![Cornell](https://github.com/sean-h/raytracer/blob/master/screenshots/cornell.png)
![Spheres](https://github.com/sean-h/raytracer/blob/master/screenshots/spheres.png)