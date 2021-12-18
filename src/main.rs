use std::{fs::File, io::Write};

use vec3::Vec3;
use ray::Ray;

mod vec3;
mod ray;


#[allow(non_snake_case)]
fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
	let oc = r.origin() - *center;

	let a = r.direction().dot(&r.direction());
	let b = 2.0 * oc.dot(&r.direction());
	let c = oc.dot(&oc) - radius.powi(2);

	let D = b.powi(2) - 4.0 * a * c;
	D > 0.0
}

fn color(r: Ray) -> Vec3 {
	if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
		return Vec3::new(1.0, 0.0, 0.0)
	}
	let unit_direction = r.direction().unit_vector();
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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

	for j in (0..NY).rev() {
		for i in 0..NX { 
			let u = i as f64 / NX as f64 ;
			let v = j as f64 / NY as f64;

			let r = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical));
			let col = color(r);

			let ir = (255.99 * col.r()) as u8;
			let ig = (255.99 * col.g()) as u8;
			let ib = (255.99 * col.b()) as u8;
			file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
		}
	}

	Ok(())
}
