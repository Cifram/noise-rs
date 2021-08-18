use crate::{
    math::s_curve::quintic::Quintic,
    permutationtable::PermutationTable,
};
use core::f64;

#[inline(always)]
fn base_perlin_2d<F>(point: [f64; 2], hasher: F) -> f64
where
    F: Fn([isize; 2]) -> usize
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=2 -> sqrt(2)
    const SCALE_FACTOR: f64 = f64::consts::SQRT_2;

    let [x, y] = point;
    let flooredx = x.floor();
    let flooredy = y.floor();
    let cornerx = flooredx as isize;
    let cornery = flooredy as isize;
    let distancex = x - flooredx;
    let distancey = y - flooredy;

    macro_rules! call_gradient(
        ($x:expr, $y:expr) => {
            {
                let x = distancex - $x as f64;
                let y = distancey - $y as f64;
                match hasher([cornerx + $x, cornery + $y]) & 0b11 {
                    0 =>  x + y, // ( 1,  1)
                    1 => -x + y, // (-1,  1)
                    2 =>  x - y, // ( 1, -1)
                    3 => -x - y, // (-1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g00 = call_gradient!(0, 0);
    let g10 = call_gradient!(1, 0);
    let g01 = call_gradient!(0, 1);
    let g11 = call_gradient!(1, 1);

    let curvex = distancex.map_quintic();
    let curvey = distancey.map_quintic();

    let k0 = g00;
    let k1 = g10 - g00;
    let k2 = g01 - g00;
    let k3 = g00 + g11 - g10 - g01;
    let unscaled_result = k0 + k1 * curvex + k2 * curvey + k3 * curvex * curvey;
    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    scaled_result.clamp(-1.0, 1.0)
}

#[inline(always)]
fn base_perlin_3d<F>(point: [f64; 3], hasher: F) -> f64
where
    F: Fn([isize; 3]) -> usize
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=3 -> 2/sqrt(3)
    // sqrt() is not a const function, so use a high-precision value instead.
    // TODO: Replace fixed const values with const fn if sqrt() ever becomes a const function.
    // 2/sqrt(3) = 1.1547005383792515290182975610039149112952035025402537520372046529
    const SCALE_FACTOR: f64 = 1.154_700_538_379_251_5;

    let [x, y, z] = point;
    let flooredx = x.floor();
    let flooredy = y.floor();
    let flooredz = z.floor();
    let cornerx = flooredx as isize;
    let cornery = flooredy as isize;
    let cornerz = flooredz as isize;
    let distancex = x - flooredx;
    let distancey = y - flooredy;
    let distancez = z - flooredz;

    macro_rules! call_gradient(
        ($x:expr, $y:expr, $z:expr) => {
            {
                let x = distancex - $x as f64;
                let y = distancey - $y as f64;
                let z = distancez - $z as f64;
                match hasher([cornerx + $x, cornery + $y, cornerz + $z]) & 0b1111 {
                    0  | 12 =>  x + y    , // ( 1,  1,  0)
                    1  | 13 => -x + y    , // (-1,  1,  0)
                    2       =>  x - y    , // ( 1, -1,  0)
                    3       => -x - y    , // (-1, -1,  0)
                    4       =>  x     + z, // ( 1,  0,  1)
                    5       => -x     + z, // (-1,  0,  1)
                    6       =>  x     - z, // ( 1,  0, -1)
                    7       => -x     - z, // (-1,  0, -1)
                    8       =>      y + z, // ( 0,  1,  1)
                    9  | 14 =>     -y + z, // ( 0, -1,  1)
                    10      =>      y - z, // ( 0,  1, -1)
                    11 | 15 =>     -y - z, // ( 0, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g000 = call_gradient!(0, 0, 0);
    let g100 = call_gradient!(1, 0, 0);
    let g010 = call_gradient!(0, 1, 0);
    let g110 = call_gradient!(1, 1, 0);
    let g001 = call_gradient!(0, 0, 1);
    let g101 = call_gradient!(1, 0, 1);
    let g011 = call_gradient!(0, 1, 1);
    let g111 = call_gradient!(1, 1, 1);

    let curvex = distancex.map_quintic();
    let curvey = distancey.map_quintic();
    let curvez = distancez.map_quintic();

    let k0 = g000;
    let k1 = g100 - g000;
    let k2 = g010 - g000;
    let k3 = g001 - g000;
    let k4 = g000 + g110 - g100 - g010;
    let k5 = g000 + g101 - g100 - g001;
    let k6 = g000 + g011 - g010 - g001;
    let k7 = g100 + g010 + g001 + g111 - g000 - g110 - g101 - g011;

    let unscaled_result =
        k0 +
        k1 * curvex +
        k2 * curvey +
        k3 * curvez +
        k4 * curvex * curvey +
        k5 * curvex * curvez +
        k6 * curvey * curvez +
        k7 * curvex * curvey * curvez;

    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    scaled_result.clamp(-1.0, 1.0)
}

#[inline(always)]
fn base_perlin_4d<F>(point: [f64; 4], hasher: F) -> f64
where
    F: Fn([isize; 4]) -> usize
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    const SCALE_FACTOR: f64 = 1.0; // 1/(sqrt(N)/2), N=4 -> 2/sqrt(4) -> 2/2 -> 1

    let [x, y, z, w] = point;
    let flooredx = x.floor();
    let flooredy = y.floor();
    let flooredz = z.floor();
    let flooredw = w.floor();
    let cornerx = flooredx as isize;
    let cornery = flooredy as isize;
    let cornerz = flooredz as isize;
    let cornerw = flooredw as isize;
    let distancex = x - flooredx;
    let distancey = y - flooredy;
    let distancez = z - flooredz;
    let distancew = w - flooredw;

    macro_rules! call_gradient(
        ($x:expr, $y:expr, $z:expr, $w:expr) => {
            {
                let x = distancex - $x as f64;
                let y = distancey - $y as f64;
                let z = distancez - $z as f64;
                let w = distancew - $w as f64;
                match hasher([cornerx + $x, cornery + $y, cornerz + $z, cornerw - $w]) & 0b11111 {
                    0  | 28 =>  x + y + z    , // ( 1,  1,  1,  0)
                    1       => -x + y + z    , // (-1,  1,  1,  0)
                    2       =>  x - y + z    , // ( 1, -1,  1,  0)
                    3       =>  x + y - z    , // ( 1,  1, -1,  0)
                    4       => -x + y - z    , // (-1,  1, -1,  0)
                    5       =>  x - y - z    , // ( 1, -1, -1,  0)
                    6       =>  x - y - z    , // (-1, -1, -1,  0)
                    7  | 29 =>  x + y     + w, // ( 1,  1,  0,  1)
                    8       => -x + y     + w, // (-1,  1,  0,  1)
                    9       =>  x - y     + w, // ( 1, -1,  0,  1)
                    10      =>  x + y     - w, // ( 1,  1,  0, -1)
                    11      =>  x + y     - w, // (-1,  1,  0, -1)
                    12      =>  x + y     - w, // ( 1, -1,  0, -1)
                    13      => -x - y     - w, // (-1, -1,  0, -1)
                    14 | 30 =>  x     + z + w, // ( 1,  0,  1,  1)
                    15      => -x     + z + w, // (-1,  0,  1,  1)
                    16      =>  x     - z + w, // ( 1,  0, -1,  1)
                    17      =>  x     + z - w, // ( 1,  0,  1, -1)
                    18      =>  x     + z - w, // (-1,  0,  1, -1)
                    19      =>  x     + z - w, // ( 1,  0, -1, -1)
                    20      => -x     - z - w, // (-1,  0, -1, -1)
                    21 | 31 =>      y + z + w, // ( 0,  1,  1,  1)
                    22      =>     -y + z + w, // ( 0, -1,  1,  1)
                    23      =>      y - z + w, // ( 0,  1, -1,  1)
                    24      =>      y - z - w, // ( 0,  1,  1, -1)
                    25      =>     -y - z - w, // ( 0, -1,  1, -1)
                    26      =>  x + y + z - w, // ( 0,  1, -1, -1)
                    27      => -x + y + z - w, // ( 0, -1, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g0000 = call_gradient!(0, 0, 0, 0);
    let g1000 = call_gradient!(1, 0, 0, 0);
    let g0100 = call_gradient!(0, 1, 0, 0);
    let g1100 = call_gradient!(1, 1, 0, 0);
    let g0010 = call_gradient!(0, 0, 1, 0);
    let g1010 = call_gradient!(1, 0, 1, 0);
    let g0110 = call_gradient!(0, 1, 1, 0);
    let g1110 = call_gradient!(1, 1, 1, 0);
    let g0001 = call_gradient!(0, 0, 0, 1);
    let g1001 = call_gradient!(1, 0, 0, 1);
    let g0101 = call_gradient!(0, 1, 0, 1);
    let g1101 = call_gradient!(1, 1, 0, 1);
    let g0011 = call_gradient!(0, 0, 1, 1);
    let g1011 = call_gradient!(1, 0, 1, 1);
    let g0111 = call_gradient!(0, 1, 1, 1);
    let g1111 = call_gradient!(1, 1, 1, 1);

    let curvex = distancex.map_quintic();
    let curvey = distancey.map_quintic();
    let curvez = distancez.map_quintic();
    let curvew = distancew.map_quintic();

    let k0 = g0000;
    let k1 = g1000 - g0000;
    let k2 = g0100 - g0000;
    let k3 = g0010 - g0000;
    let k4 = g0001 - g0000;
    let k5 = g0000 + g1100 - g1000 - g0100;
    let k6 = g0000 + g1010 - g1000 - g0010;
    let k7 = g0000 + g1001 - g1000 - g0001;
    let k8 = g0000 + g0110 - g0100 - g0010;
    let k9 = g0000 + g0101 - g0100 - g0001;
    let k10 = g0000 + g0011 - g0010 - g0001;
    let k11 = g1110 + g1000 + g0100 + g0010 - g0000 - g0111 - g1011 - g1101;
    let k12 = g1101 + g1000 + g0100 + g0001 - g0000 - g0111 - g1011 - g1110;
    let k13 = g1011 + g1000 + g0010 + g0001 - g0000 - g0111 - g1101 - g1110;
    let k14 = g0111 + g0100 + g0010 + g0001 - g0000 - g1011 - g1101 - g1110;
    let k15 = g1111 + g1000 + g0100 + g0010 + g0001 - g0000 - g0111 - g1011 - g1101 - g1110;

    let unscaled_result = k0
        + k1 * curvex
        + k2 * curvey
        + k3 * curvez
        + k4 * curvew
        + k5 * curvex * curvey
        + k6 * curvex * curvez
        + k7 * curvex * curvew
        + k8 * curvey * curvez
        + k9 * curvey * curvew
        + k10 * curvez * curvew
        + k11 * curvex * curvey * curvez
        + k12 * curvex * curvey * curvew
        + k13 * curvex * curvez * curvew
        + k14 * curvey * curvez * curvew
        + k15 * curvex * curvey * curvez * curvew;

    let scaled_result = unscaled_result * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    scaled_result.clamp(-1.0, 1.0)
}

#[inline(always)]
pub fn perlin_2d(point: [f64; 2], hasher: &PermutationTable) -> f64 {
    base_perlin_2d(point, |to_hash| hasher.hash_2d(to_hash))
}

#[inline(always)]
pub fn perlin_2d_variant(point: [f64; 2], variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_2d(point, |to_hash| hasher.hash_3d([to_hash[0], to_hash[1], variant]))
}

#[inline(always)]
pub fn perlin_3d(point: [f64; 3], hasher: &PermutationTable) -> f64 {
    base_perlin_3d(point, |to_hash| hasher.hash_3d(to_hash))
}

#[inline(always)]
pub fn perlin_3d_variant(point: [f64; 3], variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_3d(point, |to_hash| hasher.hash_4d([to_hash[0], to_hash[1], to_hash[2], variant]))
}

#[inline(always)]
pub fn perlin_4d(point: [f64; 4], hasher: &PermutationTable) -> f64 {
    base_perlin_4d(point, |to_hash| hasher.hash_4d(to_hash))
}

#[inline(always)]
pub fn perlin_4d_variant(point: [f64; 4], variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_4d(point, |to_hash| hasher.hash_5d([to_hash[0], to_hash[1], to_hash[2], to_hash[3], variant]))
}
