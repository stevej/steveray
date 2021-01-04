use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Result<HitRecord, ()>;
}