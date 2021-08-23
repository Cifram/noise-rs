//! An example of using value noise

extern crate noise;

use noise::{utils::*, core::value::{value_2d, value_3d, value_4d}, permutationtable::PermutationTable};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point| value_2d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("value 2d.png");
    PlaneMapBuilder::new_fn(|point| value_3d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("value 3d.png");
    PlaneMapBuilder::new_fn(|point| value_4d(point, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("value 4d.png");
}
