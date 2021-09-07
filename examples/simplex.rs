//! An example of using simplex noise

extern crate noise;

use noise::{
    noise_image_builder::*,
    simplex_2d, simplex_3d, simplex_4d,
    PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    NoiseImageBuilder::new(|point| simplex_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("simplex 2d.png");
    NoiseImageBuilder::new(|point| simplex_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("simplex 3d.png");
    NoiseImageBuilder::new(|point| simplex_4d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("simplex 4d.png");
}
