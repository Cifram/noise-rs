use crate::{gradient, math::vectors::*, permutationtable::PermutationTable};

const TO_REAL_CONSTANT_2D: f64 = -0.211_324_865_405_187; // (1 / sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_2D: f64 = 0.366_025_403_784_439; // (sqrt(2 + 1) - 1) / 2
const TO_SIMPLEX_CONSTANT_3D: f64 = -2.0 / 3.0;

// Determined using the Mathematica code listed in the super_simplex example and find_maximum_super_simplex.nb
const NORM_CONSTANT_2D: f64 = 1.0 / 0.054_282_952_886_616_23;
const NORM_CONSTANT_3D: f64 = 1.0 / 0.086_766_400_165_536_9;

// Points taken into account for 2D:
//             (0, -1)
//                |    \
//                |      \
//                |        \
// (-1, 0) --- ( 0,  0) --- ( 1,  0)
//        \       |    \       |    \
//          \     |      \     |      \
//            \   |        \   |        \
//             ( 0,  1) --- ( 1,  1) --- ( 2,  1)
//                     \       |
//                       \     |
//                         \   |
//                          ( 1,  2)
#[rustfmt::skip]
const LATTICE_LOOKUP_2D: [([isize; 2], [f64; 2]); 4 * 8] =
    [([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([-1, 0], [0.788_675_134_594_813_f64, -0.211_324_865_405_187_f64]),
     ([0, -1], [-0.211_324_865_405_187_f64, 0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),
     ([0, -1], [-0.211_324_865_405_187_f64, 0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([2, 1], [-1.366_025_403_784_439_f64, -0.366_025_403_784_439_04_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([-1, 0], [0.788_675_134_594_813_f64, -0.211_324_865_405_187_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),
     ([1, 2], [-0.366_025_403_784_439_04_f64, -1.366_025_403_784_439_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([1, 0], [-0.788_675_134_594_813_f64, 0.211_324_865_405_187_f64]),
     ([0, 1], [0.211_324_865_405_187_f64, -0.788_675_134_594_813_f64]),

     ([0, 0], [0_f64, 0_f64]),
     ([1, 1], [-0.577_350_269_189_626_f64, -0.577_350_269_189_626_f64]),
     ([2, 1], [-1.366_025_403_784_439_f64, -0.366_025_403_784_439_04_f64]),
     ([1, 2], [-0.366_025_403_784_439_04_f64, -1.366_025_403_784_439_f64])];

#[rustfmt::skip]
const LATTICE_LOOKUP_3D: [[isize; 3]; 4 * 16] =
    [[0, 0, 0],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[0, 0, 1],
     [0, 0, 0],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[0, 1, 0],[1, 1, 0],
     [0, 0, 0],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[1, 0, 0],[1, 0, 1],[1, 1, 0],
     [0, 0, 0],[0, 1, 1],[1, 0, 1],[1, 1, 0],
     [1, 1, 1],[0, 1, 1],[1, 0, 1],[1, 1, 0]];

pub fn super_simplex_2d(point: Vector2<f64>, hasher: &PermutationTable) -> f64 {
    // Using the vector struct internally causes a significant performance
    // regression, so break out it's components here and use those.
    let (x, y) = point.into();

    // Transform point from real space to simplex space
    let to_simplex_offset = (x + y) * TO_SIMPLEX_CONSTANT_2D;
    let simplex_point_x = x + to_simplex_offset;
    let simplex_point_y = y + to_simplex_offset;

    // Get base point of simplex and barycentric coordinates in simplex space
    let simplex_floor_x = simplex_point_x.floor();
    let simplex_floor_y = simplex_point_y.floor();
    let simplex_cell_x = simplex_floor_x as isize;
    let simplex_cell_y = simplex_floor_y as isize;
    let simplex_rel_x = simplex_point_x - simplex_floor_x;
    let simplex_rel_y = simplex_point_y - simplex_floor_y;

    // Create index to lookup table from barycentric coordinates
    let region_sum = (simplex_rel_x + simplex_rel_y).floor();
    let index =
        ((region_sum >= 1.0) as usize) << 2 |
        ((simplex_rel_x - simplex_rel_y * 0.5 + 1.0 - region_sum * 0.5 >= 1.0) as usize) << 3 |
        ((simplex_rel_y - simplex_rel_x * 0.5 + 1.0 - region_sum * 0.5 >= 1.0) as usize) << 4;

    // Transform barycentric coordinates to real space
    let to_real_offset = (simplex_rel_x + simplex_rel_y) * TO_REAL_CONSTANT_2D;
    let real_rel_x = simplex_rel_x + to_real_offset;
    let real_rel_y = simplex_rel_y + to_real_offset;

    let mut value = 0.0;

    for lattice_lookup in &LATTICE_LOOKUP_2D[index..index + 4] {
        let [latticex, latticey] = lattice_lookup.1;
        let dposx = real_rel_x + latticex;
        let dposy = real_rel_y + latticey;
        let attn = (2.0 / 3.0) - (dposx*dposx + dposy*dposy);
        if attn > 0.0 {
            let lattice_point_x = lattice_lookup.0[0] + simplex_cell_x;
            let lattice_point_y = lattice_lookup.0[1] + simplex_cell_y;
            let [gradx, grady] = gradient::grad2(hasher.hash_2d([lattice_point_x, lattice_point_y]));
            value += attn.powi(4) * (gradx*dposx + grady*dposy);
        }
    }

    value * NORM_CONSTANT_2D
}

pub fn super_simplex_3d(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    // Transform point from real space to simplex space
    let to_simplex_offset = point.sum() * TO_SIMPLEX_CONSTANT_3D;
    let simplex_point1 = point.map(|v| -(v + to_simplex_offset));
    let simplex_point2 = simplex_point1.map(|v| v + 512.5);

    // Get base point of simplex and barycentric coordinates in simplex space
    let simplex_cell1 = simplex_point1.floor_to_isize();
    let simplex_floor1 = simplex_cell1.numcast().unwrap();
    let simplex_rel1 = simplex_point1 - simplex_floor1;
    let simplex_cell2 = simplex_point2.floor_to_isize();
    let simplex_floor2 = simplex_cell2.numcast().unwrap();
    let simplex_rel2 = simplex_point2 - simplex_floor2;

    // Create indices to lookup table from barycentric coordinates
    let index1 =
        ((simplex_rel1.x + simplex_rel1.y + simplex_rel1.z >= 1.5) as usize) << 2 |
        ((-simplex_rel1.x + simplex_rel1.y + simplex_rel1.z >= 0.5) as usize) << 3 |
        ((simplex_rel1.x - simplex_rel1.y + simplex_rel1.z >= 0.5) as usize) << 4 |
        ((simplex_rel1.x + simplex_rel1.y - simplex_rel1.z >= 0.5) as usize) << 5;
    let index2 =
        ((simplex_rel2.x + simplex_rel2.y + simplex_rel2.z >= 1.5) as usize) << 2 |
        ((-simplex_rel2.x + simplex_rel2.y + simplex_rel2.z >= 0.5) as usize) << 3 |
        ((simplex_rel2.x - simplex_rel2.y + simplex_rel2.z >= 0.5) as usize) << 4 |
        ((simplex_rel2.x + simplex_rel2.y - simplex_rel2.z >= 0.5) as usize) << 5;

    let mut value = 0.0;

    // Sum contributions from first lattice
    for &lattice_lookup in &LATTICE_LOOKUP_3D[index1..index1 + 4] {
        let lattice = Vector3::from(lattice_lookup);
        let dpos = simplex_rel1 - lattice.numcast().unwrap();
        let attn = 0.75 - dpos.magnitude_squared();
        if attn > 0.0 {
            let lattice_point = simplex_cell1 + lattice;
            let gradient = Vector3::from(gradient::grad3(hasher.hash_3d(lattice_point.into())));
            value += attn.powi(4) * gradient.dot(dpos);
        }
    }

    // Sum contributions from second lattice
    for &lattice_lookup in &LATTICE_LOOKUP_3D[index2..index2 + 4] {
        let lattice = Vector3::from(lattice_lookup);
        let dpos = simplex_rel2 - lattice.numcast().unwrap();
        let attn = 0.75 - dpos.magnitude_squared();
        if attn > 0.0 {
            let lattice_point = simplex_cell2 + lattice;
            let gradient = Vector3::from(gradient::grad3(hasher.hash_3d(lattice_point.into())));
            value += attn.powi(4) * gradient.dot(dpos);
        }
    }

    value * NORM_CONSTANT_3D
}
