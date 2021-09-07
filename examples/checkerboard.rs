//! An example of generating constant valued noise

extern crate noise;

use noise::{
    checkerboard_2d, checkerboard_3d, checkerboard_4d,
    noise_image_builder::*,
};

fn main() {
    NoiseImageBuilder::new(|point| checkerboard_2d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("checkerboard 2d.png");
    NoiseImageBuilder::new(|point| checkerboard_3d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("checkerboard 3d.png");
    NoiseImageBuilder::new(|point| checkerboard_4d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("checkerboard 4d.png");
}
