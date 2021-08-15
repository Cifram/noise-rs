//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    permutationtable::PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(open_simplex_2d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("open_simplex 2d.png");
    PlaneMapBuilder::new_fn(open_simplex_3d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("open_simplex 3d.png");
    PlaneMapBuilder::new_fn(open_simplex_4d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("open_simplex 4d.png");
}
