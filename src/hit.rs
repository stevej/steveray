use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Option<Box<dyn Material>>,
}

pub trait Hitable {
    // TODO: Change Result to Option.
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Result<HitRecord, ()>;
}
