extern crate noise;

use noise::{
    core::spheres::*,
    utils::*,
};

fn main() {
    PlaneMapBuilder::new_fn(|point| spheres_2d(point.into(), 1.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("spheres 2d.png");
    PlaneMapBuilder::new_fn(|point| spheres_3d(point.into(), 2.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("spheres 3d.png");
    PlaneMapBuilder::new_fn(|point| spheres_4d(point.into(), 3.0))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("spheres 4d.png");
}
