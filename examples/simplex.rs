//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    core::simplex::{simplex_2d, simplex_3d, simplex_4d},
    permutationtable::PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point| simplex_2d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex 2d.png");
    PlaneMapBuilder::new_fn(|point| simplex_3d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex 3d.png");
    PlaneMapBuilder::new_fn(|point| simplex_4d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("simplex 4d.png");
}
