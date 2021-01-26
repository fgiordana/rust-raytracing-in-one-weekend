use std::sync::Arc;
use cgmath::{Point3, Vector3, InnerSpace, point3, vec3 };

use crate::raytracing::Ray;
use crate::material::Material;


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


