#[inline(always)]
pub fn spheres_2d(point: [f64; 2], frequency: f64) -> f64 {
    let x = point[0] * frequency;
    let y = point[1] * frequency;

    let dist_from_center = (x*x + y*y).sqrt();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}

#[inline(always)]
pub fn spheres_3d(point: [f64; 3], frequency: f64) -> f64 {
    let x = point[0] * frequency;
    let y = point[1] * frequency;
    let z = point[1] * frequency;

    let dist_from_center = (x*x + y*y + z*z).sqrt();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}

#[inline(always)]
pub fn spheres_4d(point: [f64; 4], frequency: f64) -> f64 {
    let x = point[0] * frequency;
    let y = point[1] * frequency;
    let z = point[2] * frequency;
    let w = point[3] * frequency;

    let dist_from_center = (x*x + y*y + z*z + w*w).sqrt();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}
