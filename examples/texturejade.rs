extern crate noise;

use noise::{
    permutationtable::PermutationTable,
    core::{
        displace::displace_3d,
        fbm::{fbm_ridged_perlin_3d, fbm_perlin_3d_variant},
        spheres::spheres_2d,
    },
    math::vectors::{Vector2, Vector3},
    utils::*
};

fn jade_noise(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    let point = displace_3d(point, |point, dim| fbm_perlin_3d_variant(point, dim, 4.0, 2.0, 0.5, 2, hasher) * (1.0/16.0));
    let primary_jade = fbm_ridged_perlin_3d(point, 2.0, 2.20703125, 1.0, 6, hasher);
    let secondary_point = displace_3d(point, |point, dim| fbm_perlin_3d_variant(point, dim+4, 4.0, 2.0, 0.5, 4, hasher) * (1.0/4.0));
    let secondary_point = secondary_point.rotate_axis_angle(Vector3::new(1.0, 0.2, 0.1), 84.0f64.to_radians());
    let secondary_jade = spheres_2d(Vector2::new(secondary_point.x, secondary_point.y), 2.0) * 0.25;
    primary_jade + secondary_jade
}

fn main() {
    let hasher = PermutationTable::new(0);

    let planar_texture = PlaneMapBuilder::new_fn(|point| jade_noise(point.into(), &hasher))
        .set_size(1024, 1024)
        .build();

    // Create a jade palette.
    let jade_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.000, [24, 146, 102, 255])
        .add_gradient_point(0.000, [78, 154, 115, 255])
        .add_gradient_point(0.250, [128, 204, 165, 255])
        .add_gradient_point(0.375, [78, 154, 115, 255])
        .add_gradient_point(1.000, [29, 135, 102, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(jade_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_jade_planar.png");
}
