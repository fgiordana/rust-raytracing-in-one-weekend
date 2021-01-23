use cgmath::{Vector3, InnerSpace};
use prisma::Rgb;
use rand::Rng;

use crate::hittable::{Ray, HitRecord};
use crate::util::{random_unit_vec, vec_near_zero, random_vec_in_unit_sphere};


fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - n * (2.0 * v.dot(*n))
}


fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ior_ratio: f64) -> Vector3<f64> {
    let cos_theta = n.dot(-*v).min(1.0);
    let r_out_perp = (v + n * cos_theta) * ior_ratio;
    let r_out_parallel = (-*n) * (1.0 - r_out_perp.magnitude2()).abs().sqrt();
    r_out_perp + r_out_parallel
}

fn schlick_reflectance(cosine: f64, ior: f64) -> f64 {
    let mut r0 = (1.0 - ior) / (1.0 + ior);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}


pub trait Material : Send + Sync {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Rgb<f64>,
               scattered: &mut Ray) -> bool;
}


pub struct Lambertian {
    albedo: Rgb<f64>
}

impl Lambertian {

    pub fn new(albedo: Rgb<f64>) -> Self {
        Lambertian { albedo }
    }

}

impl Material for Lambertian {

    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Rgb<f64>,
               scattered: &mut Ray) -> bool {

        let mut scatter_direction = rec.normal + random_unit_vec();

        if vec_near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray { origin: rec.p, dir: scatter_direction };
        *attenuation = self.albedo;
        true
    }

}


pub struct Metal {
    albedo: Rgb<f64>,
    fuzz: f64
}

impl Metal {

    pub fn new(albedo: Rgb<f64>, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }

}

impl Material for Metal {

    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Rgb<f64>,
               scattered: &mut Ray) -> bool {

        let reflected = reflect(&r.dir.normalize(), &rec.normal);
        *scattered = Ray { origin: rec.p, dir: reflected + random_vec_in_unit_sphere() * self.fuzz };
        *attenuation = self.albedo;
        scattered.dir.dot(rec.normal) > 0.0
    }

}


pub struct Dielectric {
    ior: f64
}

impl Dielectric {

    pub fn new(ior: f64) -> Self {
        Dielectric { ior }
    }

}

impl Material for Dielectric {

    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Rgb<f64>,
               scattered: &mut Ray) -> bool {

        *attenuation = Rgb::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ior } else { self.ior };

        let unit_direction = r.dir.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflectance = schlick_reflectance(cos_theta, refraction_ratio);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance > rand::thread_rng().gen() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray {origin: rec.p, dir: direction};
        true
    }

}


