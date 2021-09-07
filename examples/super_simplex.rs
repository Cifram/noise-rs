//! An example of using Super Simplex noise

extern crate noise;

use noise::{
    noise_image_builder::*,
    super_simplex_2d, super_simplex_3d,
    PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    NoiseImageBuilder::new(|point| super_simplex_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("super simplex 2d.png");
    NoiseImageBuilder::new(|point| super_simplex_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("super simplex 3d.png");
}
