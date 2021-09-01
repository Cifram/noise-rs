use crate::{
    math::{
        s_curve::quintic::Quintic,
        interpolate::linear,
        vectors::{Vector2, Vector3, Vector4},
    },
    permutationtable::PermutationTable,
};
use core::f64;

#[inline(always)]
fn base_perlin_2d<F>(point: Vector2<f64>, hasher: F) -> f64
where
    F: Fn([isize; 2]) -> usize
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    //
    // 1/(sqrt(N)/2), N=2 -> sqrt(2)
    const SCALE_FACTOR: f64 = f64::consts::SQRT_2;

    let corner = point.floor_to_isize();
    let floored = corner.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_gradient(
        ($offset:expr) => {
            {
                let offset = distance - $offset.numcast().unwrap();
                match hasher((corner + $offset).into()) & 0b11 {
                    0 =>  offset.x + offset.y, // ( 1,  1)
                    1 => -offset.x + offset.y, // (-1,  1)
                    2 =>  offset.x - offset.y, // ( 1, -1)
                    3 => -offset.x - offset.y, // (-1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g00 = call_gradient!(Vector2::<isize>::new(0, 0));
    let g10 = call_gradient!(Vector2::<isize>::new(1, 0));
    let g01 = call_gradient!(Vector2::<isize>::new(0, 1));
    let g11 = call_gradient!(Vector2::<isize>::new(1, 1));

    let curve = distance.map_quintic();

    let result = linear(linear(g00, g01, curve.y), linear(g10, g11, curve.y), curve.x) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
fn base_perlin_3d<F>(point: Vector3<f64>, hasher: F) -> f64
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

    let corner = point.floor_to_isize();
    let floored = corner.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_gradient(
        ($offset:expr) => {
            {
                let offset = distance - $offset.numcast().unwrap();
                match hasher((corner + $offset).into()) & 0b1111 {
                    0  | 12 =>  offset.x + offset.y    , // ( 1,  1,  0)
                    1  | 13 => -offset.x + offset.y    , // (-1,  1,  0)
                    2       =>  offset.x - offset.y    , // ( 1, -1,  0)
                    3       => -offset.x - offset.y    , // (-1, -1,  0)
                    4       =>  offset.x + offset.z, // ( 1,  0,  1)
                    5       => -offset.x + offset.z, // (-1,  0,  1)
                    6       =>  offset.x - offset.z, // ( 1,  0, -1)
                    7       => -offset.x - offset.z, // (-1,  0, -1)
                    8       =>  offset.y + offset.z, // ( 0,  1,  1)
                    9  | 14 => -offset.y + offset.z, // ( 0, -1,  1)
                    10      =>  offset.y - offset.z, // ( 0,  1, -1)
                    11 | 15 => -offset.y - offset.z, // ( 0, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g000 = call_gradient!(Vector3::<isize>::new(0, 0, 0));
    let g100 = call_gradient!(Vector3::<isize>::new(1, 0, 0));
    let g010 = call_gradient!(Vector3::<isize>::new(0, 1, 0));
    let g110 = call_gradient!(Vector3::<isize>::new(1, 1, 0));
    let g001 = call_gradient!(Vector3::<isize>::new(0, 0, 1));
    let g101 = call_gradient!(Vector3::<isize>::new(1, 0, 1));
    let g011 = call_gradient!(Vector3::<isize>::new(0, 1, 1));
    let g111 = call_gradient!(Vector3::<isize>::new(1, 1, 1));

    let curve = distance.map_quintic();

    let result = linear(
        linear(
            linear(g000, g001, curve.z),
            linear(g010, g011, curve.z),
            curve.y
        ),
        linear(
            linear(g100, g101, curve.z),
            linear(g110, g111, curve.z),
            curve.y
        ),
        curve.x
    ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
fn base_perlin_4d<F>(point: Vector4<f64>, hasher: F) -> f64
where
    F: Fn([isize; 4]) -> usize
{
    // Unscaled range of linearly interpolated perlin noise should be (-sqrt(N)/2, sqrt(N)/2).
    // Need to invert this value and multiply the unscaled result by the value to get a scaled
    // range of (-1, 1).
    const SCALE_FACTOR: f64 = 1.0; // 1/(sqrt(N)/2), N=4 -> 2/sqrt(4) -> 2/2 -> 1

    let corner = point.floor_to_isize();
    let floored = corner.numcast().unwrap();
    let distance = point - floored;

    macro_rules! call_gradient(
        ($offset:expr) => {
            {
                let offset = distance - $offset.numcast().unwrap();
                match hasher((corner + $offset).into()) & 0b11111 {
                    0  | 28 =>  offset.x + offset.y + offset.z, // ( 1,  1,  1,  0)
                    1       => -offset.x + offset.y + offset.z, // (-1,  1,  1,  0)
                    2       =>  offset.x - offset.y + offset.z, // ( 1, -1,  1,  0)
                    3       =>  offset.x + offset.y - offset.z, // ( 1,  1, -1,  0)
                    4       => -offset.x + offset.y - offset.z, // (-1,  1, -1,  0)
                    5       =>  offset.x - offset.y - offset.z, // ( 1, -1, -1,  0)
                    6       =>  offset.x - offset.y - offset.z, // (-1, -1, -1,  0)
                    7  | 29 =>  offset.x + offset.y + offset.w, // ( 1,  1,  0,  1)
                    8       => -offset.x + offset.y + offset.w, // (-1,  1,  0,  1)
                    9       =>  offset.x - offset.y + offset.w, // ( 1, -1,  0,  1)
                    10      =>  offset.x + offset.y - offset.w, // ( 1,  1,  0, -1)
                    11      =>  offset.x + offset.y - offset.w, // (-1,  1,  0, -1)
                    12      =>  offset.x + offset.y - offset.w, // ( 1, -1,  0, -1)
                    13      => -offset.x - offset.y - offset.w, // (-1, -1,  0, -1)
                    14 | 30 =>  offset.x + offset.z + offset.w, // ( 1,  0,  1,  1)
                    15      => -offset.x + offset.z + offset.w, // (-1,  0,  1,  1)
                    16      =>  offset.x - offset.z + offset.w, // ( 1,  0, -1,  1)
                    17      =>  offset.x + offset.z - offset.w, // ( 1,  0,  1, -1)
                    18      =>  offset.x + offset.z - offset.w, // (-1,  0,  1, -1)
                    19      =>  offset.x + offset.z - offset.w, // ( 1,  0, -1, -1)
                    20      => -offset.x - offset.z - offset.w, // (-1,  0, -1, -1)
                    21 | 31 =>  offset.y + offset.z + offset.w, // ( 0,  1,  1,  1)
                    22      => -offset.y + offset.z + offset.w, // ( 0, -1,  1,  1)
                    23      =>  offset.y - offset.z + offset.w, // ( 0,  1, -1,  1)
                    24      =>  offset.y - offset.z - offset.w, // ( 0,  1,  1, -1)
                    25      => -offset.y - offset.z - offset.w, // ( 0, -1,  1, -1)
                    26      =>  offset.x + offset.y + offset.z - offset.w, // ( 0,  1, -1, -1)
                    27      => -offset.x + offset.y + offset.z - offset.w, // ( 0, -1, -1, -1)
                    _ => unreachable!(),
                }
            }
        }
    );

    let g0000 = call_gradient!(Vector4::<isize>::new(0, 0, 0, 0));
    let g1000 = call_gradient!(Vector4::<isize>::new(1, 0, 0, 0));
    let g0100 = call_gradient!(Vector4::<isize>::new(0, 1, 0, 0));
    let g1100 = call_gradient!(Vector4::<isize>::new(1, 1, 0, 0));
    let g0010 = call_gradient!(Vector4::<isize>::new(0, 0, 1, 0));
    let g1010 = call_gradient!(Vector4::<isize>::new(1, 0, 1, 0));
    let g0110 = call_gradient!(Vector4::<isize>::new(0, 1, 1, 0));
    let g1110 = call_gradient!(Vector4::<isize>::new(1, 1, 1, 0));
    let g0001 = call_gradient!(Vector4::<isize>::new(0, 0, 0, 1));
    let g1001 = call_gradient!(Vector4::<isize>::new(1, 0, 0, 1));
    let g0101 = call_gradient!(Vector4::<isize>::new(0, 1, 0, 1));
    let g1101 = call_gradient!(Vector4::<isize>::new(1, 1, 0, 1));
    let g0011 = call_gradient!(Vector4::<isize>::new(0, 0, 1, 1));
    let g1011 = call_gradient!(Vector4::<isize>::new(1, 0, 1, 1));
    let g0111 = call_gradient!(Vector4::<isize>::new(0, 1, 1, 1));
    let g1111 = call_gradient!(Vector4::<isize>::new(1, 1, 1, 1));

    let curve = distance.map_quintic();

    let result =
        linear(
            linear(
                linear(
                    linear(g0000, g0001, curve.w),
                    linear(g0010, g0011, curve.w),
                    curve.z
                ),
                linear(
                    linear(g0100, g0101, curve.w),
                    linear(g0110, g0111, curve.w),
                    curve.z
                ),
                curve.y
            ),
            linear(
                linear(
                    linear(g1000, g1001, curve.w),
                    linear(g1010, g1011, curve.w),
                    curve.z
                ),
                linear(
                    linear(g1100, g1101, curve.w),
                    linear(g1110, g1111, curve.w),
                    curve.z
                ),
                curve.y
            ),
            curve.x
        ) * SCALE_FACTOR;

    // At this point, we should be really damn close to the (-1, 1) range, but some float errors
    // could have accumulated, so let's just clamp the results to (-1, 1) to cut off any
    // outliers and return it.
    result.clamp(-1.0, 1.0)
}

#[inline(always)]
pub fn perlin_2d(point: Vector2<f64>, hasher: &PermutationTable) -> f64 {
    base_perlin_2d(point, |to_hash| hasher.hash_2d(to_hash))
}

#[inline(always)]
pub fn perlin_2d_variant(point: Vector2<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_2d(point, |to_hash| hasher.hash_3d([to_hash[0], to_hash[1], variant]))
}

#[inline(always)]
pub fn perlin_3d(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    base_perlin_3d(point, |to_hash| hasher.hash_3d(to_hash))
}

#[inline(always)]
pub fn perlin_3d_variant(point: Vector3<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_3d(point, |to_hash| hasher.hash_4d([to_hash[0], to_hash[1], to_hash[2], variant]))
}

#[inline(always)]
pub fn perlin_4d(point: Vector4<f64>, hasher: &PermutationTable) -> f64 {
    base_perlin_4d(point, |to_hash| hasher.hash_4d(to_hash))
}

#[inline(always)]
pub fn perlin_4d_variant(point: Vector4<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_perlin_4d(point, |to_hash| hasher.hash_5d([to_hash[0], to_hash[1], to_hash[2], to_hash[3], variant]))
}
