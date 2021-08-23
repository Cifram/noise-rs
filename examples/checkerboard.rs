//! An example of generating constant valued noise

extern crate noise;

use noise::{
    core::checkerboard::*,
    utils::*,
};

fn main() {
    PlaneMapBuilder::new_fn(|point| checkerboard_2d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 2d.png");
    PlaneMapBuilder::new_fn(|point| checkerboard_3d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 3d.png");
    PlaneMapBuilder::new_fn(|point| checkerboard_4d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 4d.png");
}
