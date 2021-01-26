use cgmath::{Point3, InnerSpace};

use crate::animation::Animated;
use crate::raytracing::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use std::sync::Arc;


pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
    pub mat: Arc<dyn Material>
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.dir.magnitude2();
        let half_b = oc.dot(r.dir);
        let c = oc.magnitude2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        rec.p = r.at(root);
        rec.material = Some(self.mat.clone());
        rec.t = root;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        true
    }

}

impl Animated for Sphere {

    fn update(&mut self, time: f64) {}

}


pub struct AnimatedSphere {
    center0: Point3<f64>,
    center1: Point3<f64>,
    time0: f64,
    time1: f64,
    sphere: Sphere
}

impl AnimatedSphere {

    pub fn new(center0: Point3<f64>, center1: Point3<f64>, time0: f64, time1: f64, radius: f64,
               mat: Arc<dyn Material>) -> Self {
        AnimatedSphere {
            center0, center1, time0, time1, sphere: Sphere {center: center0, radius, mat}
        }
    }

}

impl Hittable for AnimatedSphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sphere.hit(r, t_min, t_max, rec)
    }

}

impl Animated for AnimatedSphere {

    fn update(&mut self, time: f64) {
        self.sphere.center = self.center0 + ((time - self.time0) / (self.time1 - self.time0))
            * (self.center1 - self.center0);
    }

}

