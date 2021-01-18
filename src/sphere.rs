use crate::vec3::Vec3;
use crate::hit::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::material::*;


pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Option<Box<dyn Material>>
}

impl Sphere {
    pub fn from(center: Vec3, radius: f64) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: None,
        }
    }
    pub fn from_with_material(center: Vec3, radius: f64, material: Option<Box<dyn Material>>) -> Self 
    {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Result<HitRecord, ()> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b*b - a*c).sqrt()) / a;
            if temp < max && temp > min {
                let p = ray.point_at(temp);
                let rec = HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                };

                return Ok(rec);
            }
            temp = (-b + (b*b - a*c).sqrt()) / a;
            if temp < max && temp > min {
                let p = ray.point_at(temp);
                let rec = HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &self.material,
                };

                return Ok(rec);
            }
        }

        return Err(());
    }
}
