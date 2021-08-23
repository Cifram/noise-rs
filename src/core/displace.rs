use crate::{
    core::perlin::{perlin_2d_variant, perlin_3d_variant, perlin_4d_variant},
    permutationtable::PermutationTable,
};

#[inline(always)]
pub fn displace_2d<NoiseF, DisplaceF>(point: [f64; 2], noise: NoiseF, displace: DisplaceF) -> f64
where
    NoiseF: Fn([f64; 2]) -> f64,
    DisplaceF: Fn([f64; 2], isize) -> f64,
{
    noise([
        point[0] + displace(point, 0),
        point[1] + displace(point, 1),
    ])
}

#[inline(always)]
pub fn displace_3d<NoiseF, DisplaceF>(point: [f64; 3], noise: NoiseF, displace: DisplaceF) -> f64
where
    NoiseF: Fn([f64; 3]) -> f64,
    DisplaceF: Fn([f64; 3], isize) -> f64,
{
    noise([
        point[0] + displace(point, 0),
        point[1] + displace(point, 1),
        point[2] + displace(point, 2),
    ])
}

#[inline(always)]
pub fn displace_4d<NoiseF, DisplaceF>(point: [f64; 4], noise: NoiseF, displace: DisplaceF) -> f64
where
    NoiseF: Fn([f64; 4]) -> f64,
    DisplaceF: Fn([f64; 4], isize) -> f64,
{
    noise([
        point[0] + displace(point, 0),
        point[1] + displace(point, 1),
        point[2] + displace(point, 2),
        point[3] + displace(point, 3),
    ])
}

#[inline(always)]
pub fn turbulance_perlin_2d<NoiseF>(point: [f64; 2], noise: NoiseF, scale: f64, hasher: &PermutationTable) -> f64
where
    NoiseF: Fn([f64; 2]) -> f64,
{
    displace_2d(point, noise, |point, dim| perlin_2d_variant(point.into(), dim+1, hasher) * scale)
}

#[inline(always)]
pub fn turbulance_perlin_3d<NoiseF>(point: [f64; 3], noise: NoiseF, scale: f64, hasher: &PermutationTable) -> f64
where
    NoiseF: Fn([f64; 3]) -> f64,
{
    displace_3d(point, noise, |point, dim| perlin_3d_variant(point.into(), dim+1, hasher) * scale)
}

#[inline(always)]
pub fn turbulance_perlin_4d<NoiseF>(point: [f64; 4], noise: NoiseF, scale: f64, hasher: &PermutationTable) -> f64
where
    NoiseF: Fn([f64; 4]) -> f64,
{
    displace_4d(point, noise, |point, dim| perlin_4d_variant(point.into(), dim+1, hasher) * scale)
}
