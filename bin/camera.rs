use rand::Rng;
use steveray::camera::Camera;
use steveray::hit::Hitable;
use steveray::hit_list::HitList;
use steveray::ray::Ray;
use steveray::sphere::Sphere;
use steveray::vec3::Vec3;

fn color<T: Hitable + Sized>(ray: Ray, world: &T) -> Vec3 {
    let rec = world.hit(ray, 0.0, f64::MAX);
    match rec {
        Ok(rec) => {
            Vec3::from(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            ) * 0.5
        }
        Err(_) => {
            let unit_direction = ray.direction().unit_vector();
            let t = (unit_direction.y() + 1.0) * 0.5;
            (Vec3::from(1.0, 1.0, 1.0) * (1.0 - t)) + Vec3::from(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255", nx, ny);
    let mut world = HitList { items: Vec::new() };
    let sphere1 = Sphere::from(Vec3::from(0.0, 0.0, -1.0), 0.5);
    world.items.push(Box::new(sphere1));
    let sphere2 = Sphere::from(Vec3::from(0.0, -100.5, -1.0), 100.0);
    world.items.push(Box::new(sphere2));
    let camera = Camera::default();

    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
            let mut col = Vec3::from(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let x = rng.gen::<f64>();
                let u = ((i as f64) + x) / (nx as f64);
                let y = rng.gen::<f64>();
                let v = ((j as f64) + y) / (ny as f64);
                let ray = camera.get_ray(u, v);
                let _p = ray.point_at(2.0);
                col += color(ray, &world);
            }
            col /= ns as f64;
            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
