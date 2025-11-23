use image::{ ImageBuffer, Rgb };
use std::time::Instant;
use num_complex::Complex;
use rayon::prelude::*;
use hsv_to_rgb::hsv_to_rgb;

fn main() {
    let image_width:u32 = 1920;
    let image_height:u32 = 1080;
    let max_iterations:u32 = 1000;

    let mut imgbuf = ImageBuffer::new(image_width, image_height);

    let x_min:f64 = -2.0;
    let x_max:f64 = 1.0;
    let y_min:f64 = -1.0;
    let y_max:f64 = 1.0;

    let start = Instant::now();

    // TODO: Calculate all pixels in parallel (based on lab 81-mandelbrot-single)

    // Placeholder for pixel calculations
    let pixels: Vec<(u32, u32, Rgb<u8>)> =
        (0..image_height)
            .into_par_iter()
            .flat_map(|y| {
                (0..image_width).into_par_iter().map(move |x| {

                    // TODO: Optimize mapping from pixel to complex plane
                    let cx = x_min + (x as f64 / image_width as f64) * (x_max - x_min);
                    let cy = y_min + (y as f64 / image_height as f64) * (y_max - y_min);

                    let c = Complex::new(cx, cy);
                    let mut z = Complex::new(0.0, 0.0);

                    let mut i = 0;

                    while i < max_iterations && z.norm() <= 2.0 {
                        z = z * z + c;
                        i += 1;
                    }

                    let pixel = if i == max_iterations {

                        Rgb([0, 0, 0])

                    } else {

                        let r = (i % 255) as u8 ;
                        let g = (i * 5 % 255) as u8 ;
                        let b = (i * 10 % 255) as u8 ;

                        Rgb([r, g, b])
                    };

                    (x , y , pixel)
                })
            })
            .collect();
    // Write pixels to image buffer
    for (x, y, pixel) in pixels {
        imgbuf.put_pixel(x, y, pixel);
    }

    let duration = start.elapsed();
    println!("Rendering time: {:?}", duration);

    std::fs::create_dir_all("./out").unwrap();
    imgbuf.save("./out/mandelbrot_multi.png").unwrap();
    println!("Image saved to ./out/mandelbrot_multi.png");
}