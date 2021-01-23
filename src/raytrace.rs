use cgmath::{InnerSpace};
use prisma::{Rgb, Lerp};

use crate::hittable::{Ray, Hittable, HitRecord};


pub fn ray_color(r: &Ray, world: &Box<dyn Hittable>, depth: i32) -> Rgb<f64> {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Rgb::new(0.0, 0.0, 0.0)
    }

    if world.hit(&r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Rgb::new(0.0, 0.0, 0.0);
        let material = rec.material.clone();
        if material.unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            let color = ray_color(&mut scattered, world, depth - 1);
            return Rgb::new(
                attenuation.red() * color.red(),
                attenuation.green() * color.green(),
                attenuation.blue() * color.blue()
            )
        }
        return Rgb::new(0.0, 0.0, 0.0)
    }

    let t = (r.dir.normalize().y + 1.0) * 0.5;
    Rgb::new(1.0, 1.0, 1.0).lerp(
        &Rgb::new(0.5, 0.7, 1.0),
        t)
}