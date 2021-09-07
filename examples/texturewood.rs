extern crate noise;

use noise::{
    PermutationTable,
    displace_3d,
    fbm_craggy_perlin_3d, fbm_perlin_3d_variant,
    spheres_2d,
    Vector2, Vector3,
    noise_image_builder::*
};

fn wood_noise(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    // Displace the wood grain texture a bit so our rings aren't completely regular.
    let point = displace_3d(point, |point, dim| fbm_perlin_3d_variant(point, dim, 2.0, 2.0, 0.5, 4, hasher) * (1.0/64.0));

    // Rotate it 84 degrees so we get a more interesting slice of the log.
    let point = point.rotate_axis_angle(Vector3::new(1.0, 0.0, 0.0), 84.0f64.to_radians());

    // Offset it a bit on the y axis to again get a more interesting slice of the log.
    let point = point + Vector3::new(0.0, 1.48, 0.0);

    // Base wood texture. Uses concentric cylinders aligned on the z-axis, like a log.
    let base_wood = spheres_2d(Vector2::new(point.x, point.y), 16.0);

    // Get a craggy noise for the wood grain.
    let wood_grain_noise = fbm_craggy_perlin_3d(point * Vector3::new(1.0, 1.0, 0.25), 48.0, 2.20703125, 0.5, 3, hasher);

    // Stretch the noise in the same direction as the concentric circles, to get a good
    // wood grain going.
    let wood_grain = wood_grain_noise * 0.25 + 0.125;

    // Add the wood grain texture to the base wood texture.
    base_wood + wood_grain
}

fn main() {
    let hasher = PermutationTable::new(0);

    // Create a wood palette.
    let wood_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [189, 94, 4, 255])
        .add_gradient_point(0.500, [144, 48, 6, 255])
        .add_gradient_point(1.0, [60, 10, 8, 255]);

    NoiseImageBuilder::new(|point| wood_noise(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_gradient(wood_gradient)
        .write_to_file("texture_wood_planar.png");
}
