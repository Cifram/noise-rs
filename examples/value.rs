//! An example of using value noise

extern crate noise;

use noise::{
    noise_image_builder::*,
    value_2d, value_3d, value_4d,
    PermutationTable,
};

fn main() {
    let hasher = PermutationTable::new(0);
    NoiseImageBuilder::new(|point| value_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("value 2d.png");
    NoiseImageBuilder::new(|point| value_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("value 3d.png");
    NoiseImageBuilder::new(|point| value_4d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("value 4d.png");
}
