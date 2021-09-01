// This is not a proper example. Rather, it's a small utility program used to
// build the look-up tables used by the worley noise functions.

// These tables each contain 256 points, evenly distributed across a 2D, 3D or
// 4D diamond, from -0.5 to 0.5. This is the largest shape within a worley cell
// that the points can be placed within, while guaranteeing that the closest
// point will never fall outside the set of cells being tested.

fn get_2d_points<F>(size: i32, filter: F) -> Vec<(f64, f64)>
where
	F: Fn(i32, i32) -> bool
{
	let mut result = Vec::new();
	let inc = 1.0 / (size - 1) as f64;
	for x in 0..size {
		for y in 0..size {
			if filter(x, y) {
				result.push((
					(x as f64 * inc) * 0.9 + 0.05,
					(y as f64 * inc) * 0.9 + 0.05,
				));
			}
		}
	}
	result
}

fn get_3d_points<F>(size: i32, filter: F) -> Vec<(f64, f64, f64)>
where
	F: Fn(i32, i32, i32) -> bool
{
	let mut result = Vec::new();
	let inc = 1.0 / (size - 1) as f64;
	for x in 0..size {
		for y in 0..size {
			for z in 0..size {
				if filter(x, y, z) {
					result.push((
						(x as f64 * inc) * 0.9 + 0.05,
						(y as f64 * inc) * 0.9 + 0.05,
						(z as f64 * inc) * 0.9 + 0.05,
					));
				}
			}
		}
	}
	result
}

fn get_4d_points<F>(size: i32, filter: F) -> Vec<(f64, f64, f64, f64)>
where
	F: Fn(i32, i32, i32, i32) -> bool
{
	let mut result = Vec::new();
	let inc = 1.0 / (size - 1) as f64;
	for x in 0..size {
		for y in 0..size {
			for z in 0..size {
				for w in 0..size {
					if filter(x, y, z, w) {
						result.push((
							(x as f64 * inc) * 0.9 + 0.05,
							(y as f64 * inc) * 0.9 + 0.05,
							(z as f64 * inc) * 0.9 + 0.05,
							(w as f64 * inc) * 0.9 + 0.05,
						));
					}
				}
			}
		}
	}
	result
}

fn filter_2d_diamond(size: i32, x: i32, y: i32) -> bool {
	let half = size/2;
	let max = size-1;
	if x < 0 || x > max || y < 0 || y > max {
		return false;
	}
	if x < half && y < half && x + y < half {
		return false;
	}
	if x > half && y < half && (max - x) + y < half {
		return false;
	}
	if x < half && y > half && x + (max - y) < half {
		return false;
	}
	if x > half && y > half && (max - x) + (max - y) < half {
		return false;
	}
	true
}

fn filter_3d_diamond(size: i32, x: i32, y: i32, z: i32) -> bool {
	if z < 0 || z > size-1 {
		return false;
	}
	let half = size/2;
	if z <= half {
		let newsize = z*2 + 1;
		let offset = (size - newsize)/2;
		filter_2d_diamond(newsize, x - offset, y - offset)
	} else {
		let newsize = (size - z - 1)*2 + 1;
		let offset = (size - newsize)/2;
		filter_2d_diamond(newsize, x - offset, y - offset)
	}
}

fn filter_4d_diamond(size: i32, x: i32, y: i32, z: i32, w: i32) -> bool {
	if w < 0 || w > size-1 {
		return false;
	}
	let half = size/2;
	if w <= half {
		let newsize = w*2 + 1;
		let offset = (size - newsize)/2;
		filter_3d_diamond(newsize, x - offset, y - offset, z - offset)
	} else {
		let newsize = (size - w - 1)*2 + 1;
		let offset = (size - newsize)/2;
		filter_3d_diamond(newsize, x - offset, y - offset, z - offset)
	}
}

fn main() {
	let points_2d = get_2d_points(23, |x, y| {
		if x >= 10 && x <= 12 && y >= 10 && y <=12 {
			return false;
		}
		filter_2d_diamond(23, x, y)
	});

	let points_3d = get_3d_points(13, |x, y, z| {
		if (x == 5 || x == 7) && (y == 5 || y == 7) && (z == 5 || z == 7) {
			return true;
		}
		if filter_3d_diamond(9, x-2, y-2, z-2) {
			return false;
		}
		filter_3d_diamond(13, x, y, z)
	});

	let points_4d = get_4d_points(7, |x, y, z, w| {
		if x == 3 && y == 3 && z == 3 && w == 3 {
			return false;
		}
		filter_4d_diamond(7, x, y, z, w)
	});
	let points_4d: Vec<(f64, f64, f64, f64)> = points_4d.iter().cloned().chain(points_4d.iter().cloned()).collect();

	println!("const WORLEY_POINTS_2D: [(f64, f64); {}] = [", points_2d.len());
	for point in points_2d.iter() {
		println!("    ({:20.17}, {:20.17}),", point.0, point.1);
	}
	println!("];");

	println!("const WORLEY_POINTS_3D: [(f64, f64, f64); {}] = [", points_3d.len());
	for point in points_3d.iter() {
		println!("    ({:20.17}, {:20.17}, {:20.17}),", point.0, point.1, point.2);
	}
	println!("];");

	println!("const WORLEY_POINTS_4D: [(f64, f64, f64, f64); {}] = [", points_4d.len());
	for point in points_4d.iter() {
		println!("    ({:20.17}, {:20.17}, {:20.17}, {:20.17}),", point.0, point.1, point.2, point.3);
	}
	println!("];");
}
