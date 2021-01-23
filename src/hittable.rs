use std::vec::Vec;
use std::sync::Arc;
use cgmath::{Point3, Vector3, InnerSpace, point3, vec3 };

use crate::material::Material;

pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>
}

impl Ray {

    pub fn new() -> Self {
        Ray {origin: point3(0.0, 0.0, 0.0), dir: vec3(0.0, 0.0, 0.0)}
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.origin + self.dir * t
    }

}


#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {

    pub fn new() -> Self {
        HitRecord {
            p: point3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            material: None,
            t: 0.0,
            front_face: false
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        self.front_face = r.dir.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }

}


pub trait Hittable : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}


pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl Hittable for HittableList {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::new();
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

}



