use std::{fs::File, io::Write};

mod vec3;

fn main() -> std::io::Result<()> {
	const NX: u8 = 200;
	const NY: u8 = 100;

	let mut file = File::create("renders/image.ppm")?;
	file.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

	for j in (0..NY).rev() {
		for i in 0..NX {
			let col = vec3::Vec3::new(i as f64 / NX as f64, j as f64 / NY as f64, 0.2);
			let ir = (255.99 * col.r()) as u8;
			let ig = (255.99 * col.g()) as u8;
			let ib = (255.99 * col.b()) as u8;
			file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
		}
	}

	Ok(())
}
