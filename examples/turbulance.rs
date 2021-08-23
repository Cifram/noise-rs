//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    core::displace::{turbulance_perlin_2d, turbulance_perlin_3d, turbulance_perlin_4d},
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    permutationtable::PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point| turbulance_perlin_2d(point, |point| perlin_2d(point.into(), &hasher), 1.0, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 2d.png");
    PlaneMapBuilder::new_fn(|point| turbulance_perlin_3d(point, |point| perlin_3d(point.into(), &hasher), 1.0, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 3d.png");
    PlaneMapBuilder::new_fn(|point| turbulance_perlin_4d(point, |point| perlin_4d(point.into(), &hasher), 1.0, &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 4d.png");
}
