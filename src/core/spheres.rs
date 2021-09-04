use crate::math::vectors::{Vector2, Vector3, Vector4};

#[inline(always)]
pub fn spheres_2d(point: Vector2<f64>, frequency: f64) -> f64 {
    let point = point * frequency;

    let dist_from_center = point.magnitude();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}

#[inline(always)]
pub fn spheres_3d(point: Vector3<f64>, frequency: f64) -> f64 {
    let point = point * frequency;

    let dist_from_center = point.magnitude();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}

#[inline(always)]
pub fn spheres_4d(point: Vector4<f64>, frequency: f64) -> f64 {
    let point = point * frequency;

    let dist_from_center = point.magnitude();

    let dist_from_smallers_sphere = dist_from_center - dist_from_center.floor();
    let dist_from_largers_sphere = 1.0 - dist_from_smallers_sphere;
    let nearest_dist = dist_from_smallers_sphere.min(dist_from_largers_sphere);

    1.0 - (nearest_dist * 4.0)
}
