use crate::hit::{HitRecord, Hitable};
use crate::ray::Ray;

pub struct HitList {
    pub items: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitList {
    fn hit(&self, ray: Ray, min: f64, max: f64) -> Result<HitRecord, ()> {
        let mut rv = Err(());
        let mut closest_so_far = max;
        for item in &self.items {
            let hitable = item.hit(ray, min, closest_so_far);
            match &hitable {
                Ok(item) => {
                    closest_so_far = item.t;
                    rv = hitable
                }
                _ => {}
            }
        }
        return rv;
    }
}
