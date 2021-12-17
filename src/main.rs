use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
	const NX: u8 = 200;
	const NY: u8 = 100;
	const IB: u8 = (255.99 * 0.2) as u8;

	let mut file = File::create("renders/image.ppm")?;
	file.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

	for j in (0..NY).rev() {
		for i in 0..NX {
			let r = i as f32 / NX as f32;
			let g = j as f32 / NY as f32;
			let ir = (255.99 * r) as u8;
			let ig = (255.99 * g) as u8;
			file.write_all(format!("{} {} {}\n", ir, ig, IB).as_bytes())?;
		}
	}

	Ok(())
}
