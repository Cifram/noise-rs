extern crate noise;

use noise::{
    permutationtable::PermutationTable,
    core::{
        displace::displace_3d,
        fbm::{fbm_ridged_perlin_3d, fbm_perlin_3d_variant},
        perlin::{perlin_3d, perlin_3d_variant},
    },
    math::{vectors::Vector3},
    select,
    utils::*,
};

#[inline]
fn slime_noise(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    // Displace the input coordinates to make the slime look turbulant
    let point = displace_3d(point, |point, dim| fbm_perlin_3d_variant(point, dim, 8.0, 2.0, 0.5, 3, hasher) * (1.0/32.0));

    // Build a slime map, which determines where to use large or small slime bubbles
    let slime_map = fbm_ridged_perlin_3d(point, 2.0, 2.20703125, 1.0, 3, hasher);

    // Select between the large or small slime bubbles based on the slime_map.
    // If it's less than -0.875 or greater than 0.875, it'll use the large bubbles.
    // Otherwise, it'll blend to the small bubbles at 0.0, and then blend back to
    // the large bubbles again.
    select!(slime_map,
        {
            // Large slime bubble texture
            perlin_3d(point*4.0, hasher).abs()*4.0 - 2.0
        } => -0.875, 0.000 => {
            // Small slime bubble texture
            perlin_3d_variant(point*24.0, 1, hasher).abs()*2.0 - 1.5
        } =>  0.000, 0.875 => {
            // Large slime bubble texture
            perlin_3d(point*4.0, hasher).abs()*4.0 - 2.0
        }
    )
}

fn main() {
    let hasher = PermutationTable::new(0);

    let planar_texture = PlaneMapBuilder::new_fn(|point| slime_noise(point.into(), &hasher))
        .set_size(1024, 1024)
        .build();

    let seamless_texture = PlaneMapBuilder::new_fn(|point| slime_noise(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_is_seamless(true)
        .build();

    // Create a slime palette.
    let slime_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.0, [160, 64, 42, 255])
        .add_gradient_point(0.0, [64, 192, 64, 255])
        .add_gradient_point(1.0, [128, 255, 128, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(slime_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_slime_planar.png");

    renderer
        .render(&seamless_texture)
        .write_to_file("texture_slime_seamless.png");
}
