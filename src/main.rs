use std::{fs::File, io::Write};

use hitable::{HitRecord, Hitable};
use ray::Ray;
use vec3::Vec3;

use crate::{hitable_list::HitableList, sphere::Sphere};

mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

fn color<H: Hitable>(r: Ray, world: &H) -> Vec3 {
    let rec = HitRecord{..Default::default()};
    if world.hit(&r, 0.0, f64::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> std::io::Result<()> {
    const NX: u8 = 200;
    const NY: u8 = 100;

    let mut file = File::create("renders/image.ppm")?;
    file.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

	let world = HitableList{ hitables: vec![
		Box::new(Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}),
		Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0}),
	]};

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;

            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical),
            );
            let col = color(r, &world);

            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
