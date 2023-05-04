# MicroPNM 🎨💻

[![Crates.io](https://img.shields.io/crates/v/micropnm)](https://crates.io/crates/micropnm)
[![GitHub](https://img.shields.io/github/license/arnemileswinter/MicroPNM)](https://github.com/arnemileswinter/MicroPNM/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.50%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub issues](https://img.shields.io/github/issues/arnemileswinter/MicroPNM)](https://github.com/arnemileswinter/MicroPNM/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/arnemileswinter/MicroPNM)](https://github.com/arnemileswinter/MicroPNM/pulls)


MicroPNM is a small and efficient library for parsing PNM image formats in Rust. 🦀
It is designed to be minimalistic and highly optimized for resource-constrained systems, 🔍
making it suitable for embedded contexts and WebAssembly using the `include_bytes!` macro. 💪

At the moment, only reading of binary PPM (P6) is supported. 🚫

## Usage 🛠️

Add the following to your `Cargo.toml` file: 

```toml
[dependencies]
micropnm = "0.1.0"
```

In your Rust code, use it like this: 

```rust
use micropnm::PNMImage;

let raw_img = include_bytes!("./path/to/your/binary_image.ppm");
let ppm_img = PNMImage::from_parse(raw_img).unwrap();

// Get the image dimensions
let width = ppm_img.width();
let height = ppm_img.height();

// Get the maximum pixel value
let max_pixel = ppm_img.maximum_pixel();

// Get the image comment
let comment = ppm_img.comment();

// Get the RGB value of a pixel
let (r, g, b) = ppm_img.pixel_rgb(10, 20).unwrap();
```

## Minimal Allocations 🧑‍💻

MicroPNM is designed with minimal memory usage in mind. 💭
This makes it suitable for use in embedded contexts or in WebAssembly modules. 🕸️
The PNMImage type itself is simply a thin wrapper around a byte slice of raw image data. 💾

## License 📜

This library is licensed under the MIT license.
