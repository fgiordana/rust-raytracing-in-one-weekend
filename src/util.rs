use cgmath::{Vector3, InnerSpace, vec3};
use prisma::Rgb;
use rand::{Rng, thread_rng};
use std::ops::Range;
use num::clamp;


pub fn random_vec_in_unit_sphere() -> Vector3<f64> {
    loop {
        let v = vec3(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0)
        );
        if v.magnitude2() < 1.0 {
            return v
        }
    }
}


pub fn random_vec_in_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = random_vec_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}


pub fn random_vec_in_unit_disk() -> Vector3<f64>{
    loop {
        let v = vec3(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            0.0
        );
        if v.magnitude2() < 1.0 {
            return v
        }
    }
}


pub fn random_unit_vec() -> Vector3<f64> {
    random_vec_in_unit_sphere().normalize()
}


pub fn random_color() -> Rgb<f64> {
    Rgb::new(
        thread_rng().gen(),
        thread_rng().gen(),
        thread_rng().gen()
    )
}


pub fn random_color_range(range: Range<f64>) -> Rgb<f64> {
    Rgb::new(
        thread_rng().gen_range(range.clone()),
        thread_rng().gen_range(range.clone()),
        thread_rng().gen_range(range)
    )
}


pub fn vec_near_zero(v: Vector3<f64>) -> bool {
    const S: f64 = 1e-8;
    v.x.abs() < S && v.y.abs() < S && v.z.abs() < S
}


pub fn to_color(value: &Rgb<f64>, num_samples: usize) -> u32 {
    let scale = 1.0 / num_samples as f64;
    let r = (value.red() * scale).sqrt();
    let g = (value.green() * scale).sqrt();
    let b = (value.blue() * scale).sqrt();

    (((256.0 * clamp(r, 0.0, 0.999)) as u32) << 16) |
        (((256.0 * clamp(g, 0.0, 0.999)) as u32) << 8) |
        ((256.0 * clamp(b, 0.0, 0.999)) as u32)
}


pub fn to_rgb(color: u32) -> [u8; 3] {
    [
        (color >> 16) as u8,
        (color >> 8) as u8,
        color as u8
    ]
}


pub fn add_colors(a: &Rgb<f64>, b: &Rgb<f64>) -> Rgb<f64> {
    Rgb::new(
        a.red() + b.red(),
        a.green() + b.green(),
        a.blue() + b.blue()
    )
}
