use prisma::Rgb;
use itertools::Itertools;
use rayon::prelude::*;
use rand::{Rng, thread_rng};

use crate::hittable::Hittable;
use crate::scene::{SceneObject, World};
use crate::camera::Camera;
use crate::raytracing::ray_color;
use crate::util::{add_colors, to_color};
use crate::animation::Animated;


pub struct Config {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    time_samples: usize,
    max_depth: usize
}

impl Config {
    pub fn new(image_width: usize, image_height: usize, samples_per_pixel: usize,
        time_samples: usize, max_depth: usize) -> Self {
        Config {image_width, image_height, samples_per_pixel, time_samples, max_depth}
    }
}


pub fn render(world: &mut World, camera: &Camera, time0: f64, time1: f64,
              config: &Config) -> Vec<u32> {

    let black = Rgb::new(0.0, 0.0, 0.0);
    let mut image = vec!(black; config.image_width * config.image_height);

    let k = (time1 - time0) as f64 / config.time_samples as f64;
    for ts in 0..config.time_samples {
        let time = time0 + (ts as f64 * k);
        world.update(time);

        let buffer = render_time_sample(world, camera, time, config);

        image = image.iter().zip(buffer.iter())
            .map(|(&i, &b)| add_colors(&i, &b))
            .collect()
    }

    let n = config.samples_per_pixel * config.time_samples;
    image.iter().map(|&i| to_color(&i, n)).collect()
}


fn render_time_sample(world: &mut World, camera: &Camera, time: f64, config: &Config) -> Vec<Rgb<f64>> {

    let black = Rgb::new(0.0, 0.0, 0.0);
    let mut buffer = vec!(black; config.image_width * config.image_height);

    (0..config.image_height).cartesian_product(0..config.image_width)
        .collect::<Vec<(usize, usize)>>()
        .into_par_iter()
        .map(|coords| {
            let x = coords.1 as f64;
            let y = coords.0 as f64;

            (0..config.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = rand::thread_rng();

                    let s = (x + rng.gen::<f64>()) / (config.image_width - 1) as f64;
                    let t = 1.0 - (y + rng.gen::<f64>()) / (config.image_height - 1) as f64;
                    let r = camera.get_ray(s, t, time);
                    ray_color(&r, world as &dyn SceneObject, config.max_depth)
                })
                .reduce(|| black,
                        |a, b| add_colors(&a, &b)
                )
        })
        .collect_into_vec(&mut buffer);

    buffer
}