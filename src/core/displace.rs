use crate::{
    core::perlin::{perlin_2d_variant, perlin_3d_variant, perlin_4d_variant},
    math::vectors::{Vector2, Vector3, Vector4},
    permutationtable::PermutationTable,
};

#[inline(always)]
pub fn displace_2d<DisplaceF>(point: Vector2<f64>, displace: DisplaceF) -> Vector2<f64>
where
    DisplaceF: Fn(Vector2<f64>, isize) -> f64,
{
    Vector2::new(
        point.x + displace(point, 0),
        point.y + displace(point, 1),
    )
}

#[inline(always)]
pub fn displace_3d<DisplaceF>(point: Vector3<f64>, displace: DisplaceF) -> Vector3<f64>
where
    DisplaceF: Fn(Vector3<f64>, isize) -> f64,
{
    Vector3::new(
        point.x + displace(point, 0),
        point.y + displace(point, 1),
        point.z + displace(point, 2),
    )
}

#[inline(always)]
pub fn displace_4d<DisplaceF>(point: Vector4<f64>, displace: DisplaceF) -> Vector4<f64>
where
    DisplaceF: Fn(Vector4<f64>, isize) -> f64,
{
    Vector4::new(
        point.x + displace(point, 0),
        point.y + displace(point, 1),
        point.z + displace(point, 2),
        point.w + displace(point, 3),
    )
}

#[inline(always)]
pub fn turbulance_perlin_2d(point: Vector2<f64>, scale: f64, hasher: &PermutationTable) -> Vector2<f64> {
    displace_2d(point, |point, dim| perlin_2d_variant(point.into(), dim+1, hasher) * scale)
}

#[inline(always)]
pub fn turbulance_perlin_3d(point: Vector3<f64>, scale: f64, hasher: &PermutationTable) -> Vector3<f64> {
    displace_3d(point, |point, dim| perlin_3d_variant(point.into(), dim+1, hasher) * scale)
}

#[inline(always)]
pub fn turbulance_perlin_4d(point: Vector4<f64>, scale: f64, hasher: &PermutationTable) -> Vector4<f64> {
    displace_4d(point, |point, dim| perlin_4d_variant(point.into(), dim+1, hasher) * scale)
}
