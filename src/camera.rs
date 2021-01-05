use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn from(origin: Vec3,
        lower_left_corner: Vec3,
        horizontal: Vec3,
        vertical: Vec3) -> Camera {
            Camera {
                origin: origin,
                lower_left_corner: lower_left_corner,
                horizontal: horizontal,
                vertical: vertical,
            }
        }

    pub fn default() -> Camera {
        Camera::from(
            Vec3::from(0.0,0.0,0.0),
            Vec3::from(-2.0,-1.0,-1.0),
            Vec3::from(4.0,0.0,0.0),
            Vec3::from(0.0, 2.0, 0.0))
        }
    
        pub fn get_ray(&self, u: f64, v: f64) -> Ray {
            Ray::from(self.origin, 
                 self.lower_left_corner +
                (self.horizontal * u) + 
                (self.vertical * v) - 
                 self.origin)
        }
}