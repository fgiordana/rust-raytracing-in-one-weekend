use cgmath::{InnerSpace, point3, vec3};
use std::sync::Arc;
use prisma::Rgb;

use crate::hittable::{Hittable, HitRecord};
use crate::raytracing::Ray;
use crate::animation::Animated;
use crate::geometry::{Sphere, AnimatedSphere};
use crate::material::{Lambertian, Metal, Dielectric, Material};
use crate::util::{random_color, random_color_range};
use rand::{thread_rng, Rng};

pub trait SceneObject : Animated + Hittable {}
impl<T: Animated + Hittable> SceneObject for T {}


pub struct World {
    pub objects: Vec<Box<dyn SceneObject>>
}

impl Hittable for World {

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

impl Animated for World {

    fn update(&mut self, time: f64) {
        for mut object in &mut self.objects {
            object.update(time);
        }
    }

}



pub fn test_scene() -> World {
    let mut world = World {objects: Vec::new()};

    let material_ground = Arc::new(
        Lambertian::new(Rgb::new(0.8, 0.8, 0.0))
    );

    let material_center = Arc::new(
        Lambertian::new(Rgb::new(0.1, 0.2, 0.5))
    );

    let material_left = Arc::new(
        Dielectric::new(1.5)
    );

    let material_right = Arc::new(
        Metal::new(Rgb::new(0.8, 0.6, 0.2), 0.0)
    );

    world.objects.push(Box::new(Sphere {
        center: point3(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: material_ground.clone()
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_center.clone()
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_left.clone()
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(-1.0, 0.0, -1.0),
        radius: -0.4,
        mat: material_left.clone()
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: material_right.clone()
    }));

    world
}


trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for f64 {
    fn in_range(self, begin: f64, end: f64) -> bool {
        self >= begin && self < end
    }
}


pub fn random_scene() -> World {
    let mut world = World {objects: Vec::new()};

    world.objects.push(Box::new(Sphere {
        center: point3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Arc::new(
            Lambertian::new(Rgb::new(0.5, 0.5, 0.5))
        )
    }));

    for a in -11..11 {
        for b in -11..11 {
            let center = point3(
                a as f64 + 0.9 * thread_rng().gen::<f64>(),
                0.2,
                b as f64 + 0.9 * thread_rng().gen::<f64>()
            );
            let center2 = center + vec3(
                0.0,
                thread_rng().gen_range(0.0..0.5),
                0.0);

            if (center - point3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material = {
                    match thread_rng().gen::<f64>() {
                        x if x.in_range(0.0, 0.6) => Arc::new(Lambertian::new(
                            random_color()
                        )) as Arc<dyn Material>,
                        x if x.in_range(0.6, 0.8) => Arc::new(Metal::new(
                            random_color_range(0.5..1.0),
                            thread_rng().gen_range(0.0..0.5)
                        )) as Arc<dyn Material>,
                        _ => Arc::new(Dielectric::new(
                            1.5
                        )) as Arc<dyn Material>
                    }
                };

                world.objects.push(Box::new(AnimatedSphere::new(
                    center,
                    center2,
                    0.0,
                    1.0,
                    0.2,
                    material.clone()
                )));
            }
        }
    }

    world.objects.push(Box::new(Sphere {
        center: point3(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::new(Dielectric::new(1.5))
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::new(Lambertian::new(Rgb::new(0.4, 0.2, 0.1)))
    }));

    world.objects.push(Box::new(Sphere {
        center: point3(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::new(Metal::new(Rgb::new(0.7, 0.6, 0.5), 0.0))
    }));

    world
}