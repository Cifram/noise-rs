//! An example of generating constant valued noise

extern crate noise;

use noise::{utils::*, Checkerboard};

fn main() {
    let checker = Checkerboard::new(1);
    PlaneMapBuilder::<Checkerboard, 2>::new(checker)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 2d.png");
    PlaneMapBuilder::<Checkerboard, 3>::new(checker)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 3d.png");
    PlaneMapBuilder::<Checkerboard, 4>::new(checker)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("checkerboard 4d.png");
}
