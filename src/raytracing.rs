use cgmath::{Point3, Vector3, InnerSpace, point3, vec3};
use prisma::{Rgb, Lerp};

use crate::hittable::{Hittable, HitRecord};
use crate::scene::SceneObject;


pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>
}

impl Ray {

    pub fn new() -> Self {
        Ray {
            origin: point3(0.0, 0.0, 0.0),
            dir: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.dir * t
    }

}


pub fn ray_color(r: &Ray, object: &dyn SceneObject, depth: usize) -> Rgb<f64> {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Rgb::new(0.0, 0.0, 0.0)
    }

    if object.hit(&r, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Rgb::new(0.0, 0.0, 0.0);
        let material = rec.material.clone();
        if material.unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            let color = ray_color(&mut scattered, object, depth - 1);
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