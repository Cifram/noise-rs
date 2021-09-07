//! An example of using simplex noise

extern crate noise;

use noise::{
    noise_image_builder::*,
    open_simplex_2d, open_simplex_3d, open_simplex_4d,
    PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    NoiseImageBuilder::new(|point| open_simplex_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("open simplex 2d.png");
    NoiseImageBuilder::new(|point| open_simplex_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("open simplex 3d.png");
    NoiseImageBuilder::new(|point| open_simplex_4d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("open simplex 4d.png");
}
