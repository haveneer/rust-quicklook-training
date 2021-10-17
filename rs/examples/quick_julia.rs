//! An example of generating julia fractals using Rayon
extern crate image;
extern crate num_complex;
use rayon::prelude::*;

use image::{ImageBuffer, Rgb};
use num_complex::Complex;

const MAX_ITER: i32 = 110;
const C: Complex<f32> = num_complex::Complex::new(0.285, 0.013);
//const C: Complex<f32> = num_complex::Complex::new(-0.9, 0.27015);

#[allow(dead_code)]
fn get_continuous_color(iteration_count: i32, z: &Complex<f32>) -> Rgb<u8> {
    let continuous_index = iteration_count as f32;
    let continuous_index = continuous_index + 1.0 - f32::log2(f32::log10(z.norm()));

    let hue = 360. * (continuous_index as f32) / (MAX_ITER as f32);
    let saturation = 1.0;
    let value = if iteration_count < MAX_ITER { 1.0 } else { 0.0 }; // MAX_ITER => black
    let hsv_color = palette::Hsv::new(hue, saturation, value);
    let color = palette::Srgb::from(hsv_color);

    let quick_convert = |value| (value * 255.) as u8;

    Rgb([
        quick_convert(color.red),
        quick_convert(color.green),
        quick_convert(color.blue),
    ])
}

#[allow(dead_code)]
fn get_indexed_color(iteration_count: i32) -> Rgb<u8> {
    let iteration_count = MAX_ITER - iteration_count;
    let r = (iteration_count << 3) as u8;
    let g = (iteration_count << 5) as u8;
    let b = (iteration_count << 4) as u8;
    Rgb([r, g, b])
}

fn main() {
    // To configure yourself the thread pool size
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(8)
    //     .build_global()
    //     .unwrap();

    let width = 1200;
    let height = 1200;

    let (xmin, ymin, xsize, ysize) = (-1.2, -1.2, 2.4, 2.4);
    let scalex = xsize / width as f32;
    let scaley = ysize / height as f32;

    let pixels = (0..width * height)
        .into_par_iter()
        .map(|offset| {
            let x = offset % width;
            let y = offset / width;

            // https://fr.wikipedia.org/wiki/Ensemble_de_Julia
            let cx = x as f32 * scalex + xmin;
            let cy = y as f32 * scaley + ymin;

            let mut z = num_complex::Complex::new(cx, cy);
            let mut count = 0;
            while count < MAX_ITER && z.norm() <= 2.0 {
                z = z * z + C;
                count += 1;
            }
            let color = get_continuous_color(count, &z);
            // let color = get_indexed_color(count);
            color
        })
        .collect::<Vec<Rgb<u8>>>();

    // Create a new ImgBuf
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);

    pixels.into_iter().enumerate().for_each(|(offset, color)| {
        let x = offset as u32 % width;
        let y = offset as u32 / width;
        imgbuf.put_pixel(x, y, color);
    });

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
