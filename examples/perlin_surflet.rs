//! An example of using perlin_surflet noise

extern crate noise;

use noise::{
    noise_image_builder::*,
    perlin_surflet_2d, perlin_surflet_3d, perlin_surflet_4d,
    PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    NoiseImageBuilder::new(|point| perlin_surflet_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("perlin surflet 2d.png");
    NoiseImageBuilder::new(|point| perlin_surflet_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("perlin surflet 3d.png");
    NoiseImageBuilder::new(|point| perlin_surflet_4d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("perlin surflet 4d.png");
}
