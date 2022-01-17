use rand::Rng;
use steveray::camera::Camera;
use steveray::hit::Hitable;
use steveray::hit_list::HitList;
use steveray::material::{EmptyMaterial, Lambertian, Material, Metal};
use steveray::ray::Ray;
use steveray::sphere::Sphere;
use steveray::vec3::Vec3;

fn color<'a, T: Hitable + Sized>(ray: &Ray, world: &T, depth: i32) -> Vec3 {
    let empty_material: Box<dyn Material> = Box::new(EmptyMaterial {});
    let hit_record = world.hit(*ray, 0.001, f64::MAX);

    match hit_record {
        Ok(ref record) => {
            if depth < 50 {
                // todo: I'd like to stop this calculation early rather using an
                // EmptyMaterial that doesn't nothing.
                match record
                    .material
                    .as_ref()
                    .unwrap_or(&empty_material)
                    .scatter(ray, &record)
                {
                    Some((attenuation, scattered)) => {
                        return attenuation * color(&scattered, world, depth + 1);
                    }
                    None => {
                        return Vec3::from(0.0, 0.0, 0.0);
                    }
                }
            } else {
                return Vec3::from(0.0, 0.0, 0.0);
            }
        }
        Err(_) => {
            let unit_direction = ray.direction().unit_vector();
            let t = (unit_direction.y + 1.0) * 0.5;
            return (Vec3::from(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::from(0.5, 0.7, 1.0) * t);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let nx = 800;
    let ny = 400;
    let ns = 400;
    let mut world = HitList { items: Vec::new() };
    let sphere1 = Sphere::from_with_material(
        Vec3::from(0.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Lambertian {
            albedo: Vec3::from(0.8, 0.3, 0.3),
        })),
    );
    let sphere2 = Sphere::from_with_material(
        Vec3::from(0.0, -100.5, -1.0),
        100.0,
        Some(Box::new(Lambertian {
            albedo: Vec3::from(0.8, 0.8, 0.0),
        })),
    );
    let sphere3 = Sphere::from_with_material(
        Vec3::from(1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Metal {
            albedo: Vec3::from(0.8, 0.6, 0.2),
            fuzz: 1.0,
        })),
    );
    let sphere4 = Sphere::from_with_material(
        Vec3::from(-1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Metal {
            albedo: Vec3::from(0.8, 0.8, 0.8),
            fuzz: 0.3,
        })),
    );
    world.items.push(Box::new(sphere1));
    world.items.push(Box::new(sphere2));
    world.items.push(Box::new(sphere3));
    world.items.push(Box::new(sphere4));
    let camera = Camera::from(
        Vec3::from(0.0, 0.0, 0.0),
        Vec3::from(-2.0, -1.0, -1.0),
        Vec3::from(4.0, 0.0, 0.0),
        Vec3::from(0.0, 2.0, 0.0),
    );

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..(ny - 1)).rev() {
        for i in 0..nx {
            let mut col = Vec3::from(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let i_r = rng.gen::<f64>();
                let u = ((i as f64) + i_r) / (nx as f64);
                let j_r = rng.gen::<f64>();
                let v = ((j as f64) + j_r) / (ny as f64);
                let r = camera.get_ray(u, v);
                let _p = r.point_at(2.0);
                col += color(&r, &world, 0);
            }
            col /= ns as f64;
            col = Vec3::from(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
