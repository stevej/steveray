use steveray::vec3::Vec3;
use steveray::ray::Ray;

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::from(1.0, 1.0, 1.0)*(1.0 - t) + Vec3::from(0.5, 0.7, 1.0)*t
}

fn main() {
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3::from(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from(4.0, 0.0, 0.0);
    let vertical = Vec3::from(0.0, 2.0, 0.0);
    let origin = Vec3::from(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..(ny -1)).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray::from(origin, lower_left_corner + horizontal*u + vertical*v);
            let color = color(r);
            let ir = (255.9 * color.x()) as i32;
            let ig = (255.9 * color.y()) as i32;
            let ib = (255.9 * color.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}