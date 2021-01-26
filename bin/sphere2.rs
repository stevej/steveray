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

    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::from(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from(4.0, 0.0, 0.0);
    let vertical = Vec3::from(0.0, 2.0, 0.0);
    let origin = Vec3::from(0.0, 0.0, 0.0);

    let mut world = HitList { items: Vec::new() };
    let sphere1 = Sphere::from(Vec3::from(0.0, 0.0, -1.0), 0.5);
    world.items.push(Box::new(sphere1));
    let sphere2 = Sphere::from(Vec3::from(0.0, -100.5, -1.0), 100.0);
    world.items.push(Box::new(sphere2));
    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let ray = Ray::from(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v),
            );
            let _p = ray.point_at(2.0);
            let col = color(ray, &world);
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
