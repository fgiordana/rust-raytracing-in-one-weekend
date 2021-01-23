use std::io::{Error};
use std::{thread, time};
use std::iter::once;
use cgmath::{point3, vec3};
use minifb::{Key, Window, WindowOptions};


mod rendering;
mod raytracing;
mod scene;
mod hittable;
mod geometry;
mod camera;
mod material;
mod util;

use crate::rendering::{Config, render};
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::scene::random_scene;
use crate::util::{to_rgb};


fn main() -> Result<(), Error> {

    // Image

    let image_path = "image.png";
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Window

    let mut window = Window::new(
        "Render",
        image_width,
        image_height,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // World

    let world: Box<dyn Hittable> = Box::new(random_scene());

    // Camera

    let lookfrom = point3(13.0, 2.0, 3.0);
    let lookat = point3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        aperture,
        focus_dist
    );

    // Render

    println!("Rendering to file: {}", image_path);

    let begin_t = time::Instant::now();

    let buffer = render(&world, &camera, &Config::new(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth
    ));

    let duration = begin_t.elapsed();

    println!("Rendered in: {:.2?}", duration);


    window.update_with_buffer(&buffer, image_width, image_height)
        .unwrap();

    let rgb_buffer: Vec<u8> = buffer.iter()
        .map(|x| {
            to_rgb(*x)
        })
        .flat_map(|x| {
            once(x[0]).chain(once(x[1])).chain(once(x[2]))
        })
        .collect();

    let image_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::from_raw(
        image_width as u32,
        image_height as u32,
        rgb_buffer
    ).unwrap();

    image_buffer.save(image_path).unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        thread::sleep(time::Duration::from_millis(10));

        window.update_with_buffer(&buffer, image_width, image_height)
           .unwrap();
    }

    Ok(())
}
