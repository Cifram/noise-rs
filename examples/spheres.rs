extern crate noise;

use noise::{
    spheres_2d, spheres_3d, spheres_4d,
    noise_image_builder::*,
};

fn main() {
    NoiseImageBuilder::new(|point| spheres_2d(point.into(), 1.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("spheres 2d.png");
    NoiseImageBuilder::new(|point| spheres_3d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("spheres 3d.png");
    NoiseImageBuilder::new(|point| spheres_4d(point.into(), 3.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .write_to_file("spheres 4d.png");
}
