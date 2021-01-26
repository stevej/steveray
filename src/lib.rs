pub mod camera;
pub mod hit;
pub mod hit_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use rand::Rng;
use vec3::Vec3;

pub fn random_in_unit_sphere() -> Vec3 {
  let mut rng = rand::thread_rng();
  let mut p: Vec3;
  // Generate vectors in unit cube and test if they're in the unit sphere
  loop {
    p = Vec3::from(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
    if p.squared_length() >= 1.0 {
      break;
    }
  }
  return p;
}
