use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    // Returns a Vec3 of the attenuation and a Ray for how scattered
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Copy, Clone)]
pub struct EmptyMaterial;
// No scatter is possible
impl Material for EmptyMaterial {
    #[allow(unused_variables)]
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        return None;
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.p + hit_record.normal + super::random_in_unit_sphere();
        let scattered = Ray::from(hit_record.p, target - hit_record.p);
        return Some((self.albedo, scattered));
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        // todo: move this into a constructor
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };
        let reflected = ray_in.direction().unit_vector().reflect(hit_record.normal);
        let scattered = Ray::from(
            hit_record.p,
            reflected + (super::random_in_unit_sphere() * fuzz),
        );
        return Some((self.albedo, scattered));
    }
}
