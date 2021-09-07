extern crate noise;

use noise::{
    permutationtable::PermutationTable,
    core::{
        displace::displace_3d,
        worley::worley_3d_range,
        fbm::{fbm_perlin_3d, fbm_perlin_3d_variant},
    },
    math::vectors::Vector3,
    utils::*
};

fn granite_noise(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    // Perturb the granite texture to add realism.
    let point = displace_3d(point, |point, dim| fbm_perlin_3d_variant(point, dim, 4.0, 2.0, 0.5, 6, hasher) * (1.0/8.0));

    // Primary granite texture, this generates the "roughness" of the texture.
    let primary_granite = fbm_perlin_3d(point, 12.0, 2.18359375, 0.625, 6, hasher);

    // Square the granite texture when it's positive, to shrink the highest areas.
    let primary_granite = if primary_granite > 0.0 { primary_granite * primary_granite } else { primary_granite };

    // Use worley noise to produce small grains for the granite texture.
    let grains = worley_3d_range(point * 16.0, hasher) * -0.5;

    // Combine the primary texture with the small grain texture.
    primary_granite + grains
}

fn main() {
    let hasher = PermutationTable::new(0);
    let planar_texture = PlaneMapBuilder::new_fn(|point| granite_noise(point.into(), &hasher))
        .set_size(1024, 1024)
        .build();

    // Create a gray granite palette. Black and pink appear at either ends of the palette; these
    // colors provide the characteristic flecks in granite.
    let granite_gradient = ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-1.0000, [0, 0, 0, 255])
        .add_gradient_point(-0.9375, [0, 0, 0, 255])
        .add_gradient_point(-0.8750, [216, 216, 242, 255])
        .add_gradient_point(0.0000, [191, 191, 191, 255])
        .add_gradient_point(0.5000, [210, 116, 125, 255])
        .add_gradient_point(0.7500, [210, 113, 98, 255])
        .add_gradient_point(1.0000, [255, 176, 192, 255]);

    let mut renderer = ImageRenderer::new().set_gradient(granite_gradient);

    renderer
        .render(&planar_texture)
        .write_to_file("texture_granite_planar.png");
}
