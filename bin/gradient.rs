use steveray::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
            let vec = Vec3 {
                x: (i as f64) / (nx as f64),
                y: (j as f64) / (ny as f64),
                z: 0.2,
            };
            let ir = (255.9 * vec.x()) as i32;
            let ig = (255.9 * vec.y()) as i32;
            let ib = (255.9 * vec.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
