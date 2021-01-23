use prisma::Rgb;
use itertools::Itertools;
use rayon::prelude::*;
use rand::Rng;

use crate::hittable::Hittable;
use crate::camera::Camera;
use crate::raytracing::ray_color;
use crate::util::to_color;


pub struct Config {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize
}

impl Config {
    pub fn new(image_width: usize, image_height: usize, samples_per_pixel: usize,
        max_depth: usize) -> Self {
        Config {image_width, image_height, samples_per_pixel, max_depth}
    }
}


pub fn render(world: &Box<dyn Hittable>, camera: &Camera, config: &Config) -> Vec<u32> {

    let mut buffer = vec!(0; config.image_width * config.image_height);

    let black = Rgb::new(0.0, 0.0, 0.0);

    (0..config.image_height).cartesian_product(0..config.image_width)
        .collect::<Vec<(usize, usize)>>()
        .into_par_iter()
        .map(|coords| {
            let x = coords.1 as f64;
            let y = coords.0 as f64;

            let pixel_color = (0..config.samples_per_pixel)
                .into_par_iter()
                .map(|s| {
                    let mut rng = rand::thread_rng();
                    let u = (x + rng.gen::<f64>()) / (config.image_width - 1) as f64;
                    let v = 1.0 - (y + rng.gen::<f64>()) / (config.image_height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    ray_color(&r, world, config.max_depth)
                })
                .reduce(|| black,
                        |a, b| Rgb::new(
                            a.red() + b.red(),
                            a.green() + b.green(),
                            a.blue() + b.blue()
                        )
                );

            to_color(&pixel_color, config.samples_per_pixel)
        })
        .collect_into_vec(&mut buffer);

    buffer
}
