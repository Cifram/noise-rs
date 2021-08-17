#[inline(always)]
pub fn checkerboard_2d(point: [f64; 2], grid_size: f64) -> f64 {
    let [x, y] = point;
    let floorx = (x / grid_size).floor() as isize;
    let floory = (y / grid_size).floor() as isize;

    if (floorx & 1) ^ (floory & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}

#[inline(always)]
pub fn checkerboard_3d(point: [f64; 3], grid_size: f64) -> f64 {
    let [x, y, z] = point;
    let floorx = (x / grid_size).floor() as isize;
    let floory = (y / grid_size).floor() as isize;
    let floorz = (z / grid_size).floor() as isize;

    if (floorx & 1) ^ (floory & 1) ^ (floorz & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}

#[inline(always)]
pub fn checkerboard_4d(point: [f64; 4], grid_size: f64) -> f64 {
    let [x, y, z, w] = point;
    let floorx = (x / grid_size).floor() as isize;
    let floory = (y / grid_size).floor() as isize;
    let floorz = (z / grid_size).floor() as isize;
    let floorw = (w / grid_size).floor() as isize;

    if (floorx & 1) ^ (floory & 1) ^ (floorz & 1) ^ (floorw & 1) == 0 {
        -1.0
    } else {
        1.0
    }
}
