use crate::{
    math::{
        s_curve::quintic::Quintic,
        interpolate::linear,
    },
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

    let result = linear(linear(g00, g01, curvey), linear(g10, g11, curvey), curvex) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
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

    let result = linear(
        linear(
            linear(g000, g001, curvez),
            linear(g010, g011, curvez),
            curvey
        ),
        linear(
            linear(g100, g101, curvez),
            linear(g110, g111, curvez),
            curvey
        ),
        curvex
    ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
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
                match hasher([cornerx + $x, cornery + $y, cornerz + $z, cornerw + $w]) & 0b11111 {
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

    let result =
        linear(
            linear(
                linear(
                    linear(g0000, g0001, curvew),
                    linear(g0010, g0011, curvew),
                    curvez
                ),
                linear(
                    linear(g0100, g0101, curvew),
                    linear(g0110, g0111, curvew),
                    curvez
                ),
                curvey
            ),
            linear(
                linear(
                    linear(g1000, g1001, curvew),
                    linear(g1010, g1011, curvew),
                    curvez
                ),
                linear(
                    linear(g1100, g1101, curvew),
                    linear(g1110, g1111, curvew),
                    curvez
                ),
                curvey
            ),
            curvex
        ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
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
