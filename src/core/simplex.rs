use crate::{
    gradient,
    math::vectors::{Vector2, Vector3, Vector4},
    permutationtable::PermutationTable,
};

/// The simplex noise code was adapted from code by Stefan Gustavson,
/// http://staffwww.itn.liu.se/~stegu/aqsis/aqsis-newnoise/sdnoise1234.c
///
/// This is Stefan Gustavson's original copyright notice:
///
/// /* sdnoise1234, Simplex noise with true analytic
///  * derivative in 1D to 4D.
///  *
///  * Copyright © 2003-2011, Stefan Gustavson
///  *
///  * Contact: stefan.gustavson@gmail.com
///  *
///  * This library is public domain software, released by the author
///  * into the public domain in February 2011. You may do anything
///  * you like with it. You may even remove all attributions,
///  * but of course I'd appreciate it if you kept my name somewhere.
///  *
///  * This library is distributed in the hope that it will be useful,
///  * but WITHOUT ANY WARRANTY; without even the implied warranty of
///  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
///  * General Public License for more details.
///  */

#[inline(always)]
fn base_simplex_2d<F>(point: Vector2<f64>, hasher: F) -> (f64, Vector2<f64>)
where
    F: Fn([isize; 2]) -> usize
{
    const SKEW_FACTOR_2D: f64 = 0.366025403;
    const UNSKEW_FACTOR_2D: f64 = 0.211324865;

    // Skew the input space to determine which simplex cell we're in
    let skew = point.sum() * SKEW_FACTOR_2D;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast().unwrap();

    let unskew: f64 = floor.sum() as f64 * UNSKEW_FACTOR_2D;
    // Unskew the cell origin back to (x,y) space
    let unskewed = floor - unskew;
    // The x,y distances from the cell origin
    let corner_offset1 = point - unskewed;

    // For the 2D case, the simplex shape is an equilateral triangle.
    // Determine which simplex we are in.
    let offset = if corner_offset1.x > corner_offset1.y {
        /* Offsets for second (middle) corner of simplex in (i,j) coords */
        // lower triangle, XY order: (0,0)->(1,0)->(1,1)
        Vector2::new(1.0, 0.0)
    } else {
        // upper triangle, YX order: (0,0)->(0,1)->(1,1)
        Vector2::new(0.0, 1.0)
    };

    // A step of (1,0) in (i,j) means a step of (1-c,-c) in (x,y), and
    // a step of (0,1) in (i,j) means a step of (-c,1-c) in (x,y), where
    // c = (3-sqrt(3))/6   */
    // Offsets for middle corner in (x,y) unskewed coords
    let corner_offset2 = corner_offset1 - offset + UNSKEW_FACTOR_2D;
    // Offsets for last corner in (x,y) unskewed coords
    let corner_offset3 = corner_offset1 - 1.0 + 2.0 * UNSKEW_FACTOR_2D;

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        grad: Vector2<f64>,
    }

    #[inline(always)]
    fn surflet(grad_index: usize, point: Vector2<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let grad = gradient::grad2(grad_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(grad),
                t, t2, t4,
                grad,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0, t2: 0.0, t4: 0.0,
                grad: Vector2::zero(),
            }
        }
    }

    // Calculate gradient indexes for each corner
    let grad_index1 = hasher(cell.into());
    let grad_index2 = hasher((cell + offset.numcast().unwrap()).into());
    let grad_index3 = hasher((cell + 1).into());
    // Calculate the contribution from the three corners
    let corner1 = surflet(grad_index1, corner_offset1);
    let corner2 = surflet(grad_index2, corner_offset2);
    let corner3 = surflet(grad_index3, corner_offset3);

    // Add contributions from each corner to get the final noise value.
    // The result is scaled to return values in the interval [-1, 1].
    let noise = corner1.value + corner2.value + corner3.value;

    //  A straight, unoptimised calculation would be like:
    //    dnoise_dx = -8.0 * t20 * t0 * x0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gx0;
    //    dnoise_dy = -8.0 * t20 * t0 * y0 * ( gx0 * x0 + gy0 * y0 ) + t40 * gy0;
    //    dnoise_dx += -8.0 * t21 * t1 * x1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gx1;
    //    dnoise_dy += -8.0 * t21 * t1 * y1 * ( gx1 * x1 + gy1 * y1 ) + t41 * gy1;
    //    dnoise_dx += -8.0 * t22 * t2 * x2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gx2;
    //    dnoise_dy += -8.0 * t22 * t2 * y2 * ( gx2 * x2 + gy2 * y2 ) + t42 * gy2;
    let dnoise = (
        corner_offset1 + corner1.t2 * corner1.t * corner1.grad.dot(corner_offset1) +
        corner_offset2 + corner2.t2 * corner2.t * corner2.grad.dot(corner_offset2) +
        corner_offset2 + corner3.t2 * corner3.t * corner3.grad.dot(corner_offset3)
    ) * 8.0 + corner1.grad * corner1.t4 + corner2.grad * corner2.t4 + corner3.grad * corner3.t4;

    (noise, dnoise)
}

#[inline(always)]
fn base_simplex_3d<F>(point: Vector3<f64>, hasher: F) -> (f64, Vector3<f64>)
where
    F: Fn([isize; 3]) -> usize
{
    const SKEW_FACTOR_3D: f64 = 0.333333333;
    const UNSKEW_FACTOR_3D: f64 = 0.166666667;

    // Skew the input space to determine which simplex cell we're in
    let skew = point.sum() * SKEW_FACTOR_3D;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast::<f64>().unwrap();

    // let unskew = (cell_x + cell_y + cell_z) as f64 * UNSKEW_FACTOR_3D;
    let unskew = floor.sum() * UNSKEW_FACTOR_3D;
    /* Unskew the cell origin back to (x,y,z) space */
    let unskewed = floor - unskew;
    /* The x,y,z distances from the cell origin */
    let corner_offset1 = point - unskewed;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    /* TODO: This code would benefit from a backport from the GLSL version! */
    let (order1, order2) = if corner_offset1.x >= corner_offset1.y {
        if corner_offset1.y >= corner_offset1.z {
            /* X Y Z order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 1, 0))
        } else if corner_offset1.x >= corner_offset1.z {
            /* X Z Y order */
            (Vector3::new(1, 0, 0), Vector3::new(1, 0, 1))
        } else {
            /* Z X Y order */
            (Vector3::new(0, 0, 1), Vector3::new(1, 0, 1))
        }
    } else {
        // x0<y0
        if corner_offset1.y < corner_offset1.z {
            /* Z Y X order */
            (Vector3::new(0, 0, 1), Vector3::new(0, 1, 1))
        } else if corner_offset1.x < corner_offset1.z {
            /* Y Z X order */
            (Vector3::new(0, 1, 0), Vector3::new(0, 1, 1))
        } else {
            /* Y X Z order */
            (Vector3::new(0, 1, 0), Vector3::new(1, 1, 0))
        }
    };

    /* A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
     * a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
     * a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
     * c = 1/6.   */

    let corner_offset2 = corner_offset1 - order1.numcast().unwrap() + UNSKEW_FACTOR_3D;
    let corner_offset3 = corner_offset1 - order2.numcast().unwrap() + 2.0 * UNSKEW_FACTOR_3D;
    let corner_offset4 = corner_offset1 - 1.0 + 3.0 * UNSKEW_FACTOR_3D;

    // Calculate gradient indexes for each corner
    let gi1 = hasher(cell.into());
    let gi2 = hasher((cell + order1).into());
    let gi3 = hasher((cell + order2).into());
    let gi4 = hasher((cell + 1).into());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        grad: Vector3<f64>,
    }

    #[inline(always)]
    fn surflet(grad_index: usize, point: Vector3<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let grad = gradient::grad3(grad_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(grad),
                t, t2, t4,
                grad,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                grad: Vector3::zero(),
            }
        }
    }

    // Calculate the contribution from the four corners
    let corner1 = surflet(gi1, corner_offset1);
    let corner2 = surflet(gi2, corner_offset2);
    let corner3 = surflet(gi3, corner_offset3);
    let corner4 = surflet(gi4, corner_offset4);

    //  Add contributions from each corner to get the final noise value.
    // The result is scaled to return values in the range [-1,1]
    let noise = corner1.value + corner2.value + corner3.value + corner4.value;

    // A straight, unoptimised calculation would be like:
    //   dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gx0;
    //   dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gy0;
    //   dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gz0;
    //   dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gx1;
    //   dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gy1;
    //   dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gz1;
    //   dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gx2;
    //   dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gy2;
    //   dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gz2;
    //   dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gx3;
    //   dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gy3;
    //   dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gz3;
    let dnoise = (
        corner_offset1 * corner1.t2 * corner1.t * corner1.grad.dot(corner_offset1) +
        corner_offset2 * corner2.t2 * corner2.t * corner2.grad.dot(corner_offset2) +
        corner_offset3 * corner3.t2 * corner3.t * corner3.grad.dot(corner_offset3) +
        corner_offset4 * corner4.t2 * corner4.t * corner4.grad.dot(corner_offset4)
    ) * -8.0 + corner1.grad * corner1.t4 + corner2.grad * corner2.t4 + corner3.grad * corner3.t4 + corner4.grad * corner4.t4;

    (noise, dnoise)
}

#[inline(always)]
fn base_simplex_4d<F>(point: Vector4<f64>, hasher: F) -> (f64, Vector4<f64>)
where
    F: Fn([isize; 4]) -> usize
{
    const SKEW_FACTOR_4D: f64 = 0.309016994;
    const UNSKEW_FACTOR_4D: f64 = 0.138196601;

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    // Factor for 4D skewing
    let skew = point.sum() * SKEW_FACTOR_4D;
    let skewed = point + skew;
    let cell = skewed.floor_to_isize();
    let floor = cell.numcast::<f64>().unwrap();

    // Factor for 4D unskewing
    let unskew = floor.sum() * UNSKEW_FACTOR_4D;
    // Unskew the cell origin back to (x,y,z,w) space
    let unskewed = floor - unskew;

    // The x,y,z,w distances from the cell origin
    let corner_offset1 = point - unskewed;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = if corner_offset1.x > corner_offset1.y { 32 } else { 0 };
    let c2 = if corner_offset1.x > corner_offset1.z { 16 } else { 0 };
    let c3 = if corner_offset1.y > corner_offset1.z { 8 } else { 0 };
    let c4 = if corner_offset1.x > corner_offset1.w { 4 } else { 0 };
    let c5 = if corner_offset1.y > corner_offset1.w { 2 } else { 0 };
    let c6 = if corner_offset1.z > corner_offset1.w { 1 } else { 0 };
    let c = c1 | c2 | c3 | c4 | c5 | c6; // '|' is mostly faster than '+'

    // These are the integer offsets of the other 3 simplex corners.
    // simplex[c] is a 4-vector with the numbers 0, 1, 2 and 3 in some order.
    // Many values of c will never occur, since e.g. x>y>z>w makes x<z, y<w and x<w
    // impossible. Only the 24 indices which have non-zero entries make any sense.
    // We use a thresholding to set the coordinates in turn from the largest magnitude.
    // The number 3 in the "simplex" array is at the position of the largest coordinate.
    let order1 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 3 { 1 } else { 0 });
    // The number 2 in the "simplex" array is at the second largest coordinate.
    let order2 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 2 { 1 } else { 0 });
    // The number 1 in the "simplex" array is at the second smallest coordinate.
    let order3 = Vector4::from(SIMPLEX[c]).map(|n| if n >= 1 { 1 } else { 0 });
    // The fifth corner has all coordinate offsets = 1, so no need to look that up.

    // Offsets for second corner in (x,y,z,w) coords
    let corner_offset2 = corner_offset1 - order1.numcast().unwrap() + UNSKEW_FACTOR_4D;
    // Offsets for third corner in (x,y,z,w) coords
    let corner_offset3 = corner_offset1 - order2.numcast().unwrap() + 2.0 * UNSKEW_FACTOR_4D;
    // Offsets for fourth corner in (x,y,z,w) coords
    let corner_offset4 = corner_offset1 - order3.numcast().unwrap() + 3.0 * UNSKEW_FACTOR_4D;
    // Offsets for last corner in (x,y,z,w) coords
    let corner_offset5 = corner_offset1 - 1.0 + 4.0 * UNSKEW_FACTOR_4D;

    // Calculate gradient indexes for each corner
    let gi1 = hasher(cell.into());
    let gi2 = hasher((cell + order1).into());
    let gi3 = hasher((cell + order2).into());
    let gi4 = hasher((cell + order3).into());
    let gi5 = hasher((cell + 1).into());

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        grad: Vector4<f64>,
    }

    #[inline(always)]
    fn surflet(grad_index: usize, point: Vector4<f64>) -> SurfletComponents {
        let t = 1.0 - point.magnitude_squared() * 2.0;

        if t > 0.0 {
            let grad = gradient::grad4(grad_index).into();
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: (2.0 * t2 + t4) * point.dot(grad),
                t, t2, t4,
                grad,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0, t2: 0.0, t4: 0.0,
                grad: Vector4::zero(),
            }
        }
    }

    // Calculate the contribution from the five corners
    let corner1 = surflet(gi1, corner_offset1);
    let corner2 = surflet(gi2, corner_offset2);
    let corner3 = surflet(gi3, corner_offset3);
    let corner4 = surflet(gi4, corner_offset4);
    let corner5 = surflet(gi5, corner_offset5);

    // Sum up and scale the result to cover the range [-1,1]
    let noise = corner1.value + corner2.value + corner3.value + corner4.value + corner5.value; // TODO: The scale factor is preliminary!

    // A straight, unoptimised calculation would be like:
    //   dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gx0;
    //   dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gy0;
    //   dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gz0;
    //   dnoise_dw = -8.0 * t20 * t0 * w0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gw0;
    //   dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gx1;
    //   dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gy1;
    //   dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gz1;
    //   dnoise_dw += -8.0 * t21 * t1 * w1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gw1;
    //   dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gx2;
    //   dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gy2;
    //   dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gz2;
    //   dnoise_dw += -8.0 * t22 * t2 * w2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gw2;
    //   dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gx3;
    //   dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gy3;
    //   dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gz3;
    //   dnoise_dw += -8.0 * t23 * t3 * w3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gw3;
    //   dnoise_dx += -8.0 * t24 * t4 * x4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gx4;
    //   dnoise_dy += -8.0 * t24 * t4 * y4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gy4;
    //   dnoise_dz += -8.0 * t24 * t4 * z4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gz4;
    //   dnoise_dw += -8.0 * t24 * t4 * w4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gw4;
    let dnoise = (
        corner_offset1 * corner1.t2 * corner1.t * corner1.grad.dot(corner_offset1) +
        corner_offset2 * corner2.t2 * corner2.t * corner2.grad.dot(corner_offset2) +
        corner_offset3 * corner3.t2 * corner3.t * corner3.grad.dot(corner_offset3) +
        corner_offset4 * corner4.t2 * corner4.t * corner4.grad.dot(corner_offset4) +
        corner_offset5 * corner5.t2 * corner5.t * corner5.grad.dot(corner_offset5)
    ) * -8.0 + corner1.grad * corner1.t4 + corner2.grad * corner2.t4 + corner3.grad * corner3.t4 + corner4.grad * corner4.t4 + corner5.grad * corner5.t4;

    (noise, dnoise)
}

// A lookup table to traverse the simplex around a given point in 4D.
// Details can be found where this table is used, in the 4D noise method.
/* TODO: This should not be required, backport it from Bill's GLSL code! */
#[rustfmt::skip]
const SIMPLEX: [[u8; 4]; 64] = [
    [0, 1, 2, 3], [0, 1, 3, 2], [0, 0, 0, 0], [0, 2, 3, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 2, 3, 0],
    [0, 2, 1, 3], [0, 0, 0, 0], [0, 3, 1, 2], [0, 3, 2, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [1, 3, 2, 0],
    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
    [1, 2, 0, 3], [0, 0, 0, 0], [1, 3, 0, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 3, 0, 1], [2, 3, 1, 0],
    [1, 0, 2, 3], [1, 0, 3, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [2, 0, 3, 1], [0, 0, 0, 0], [2, 1, 3, 0],
    [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],
    [2, 0, 1, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [3, 0, 1, 2], [3, 0, 2, 1], [0, 0, 0, 0], [3, 1, 2, 0],
    [2, 1, 0, 3], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [3, 1, 0, 2], [0, 0, 0, 0], [3, 2, 0, 1], [3, 2, 1, 0],
];

#[inline(always)]
pub fn simplex_2d(point: Vector2<f64>, hasher: &PermutationTable) -> f64 {
    base_simplex_2d(point, |to_hash| hasher.hash_2d(to_hash)).0
}

#[inline(always)]
pub fn simplex_2d_deriv(point: Vector2<f64>, hasher: &PermutationTable) -> (f64, Vector2<f64>) {
    base_simplex_2d(point, |to_hash| hasher.hash_2d(to_hash))
}

#[inline(always)]
pub fn simplex_2d_variant(point: Vector2<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_simplex_2d(point, |to_hash| hasher.hash_3d([to_hash[0], to_hash[1], variant])).0
}

#[inline(always)]
pub fn simplex_2d_variant_deriv(point: Vector2<f64>, variant: isize, hasher: &PermutationTable) -> (f64, Vector2<f64>) {
    base_simplex_2d(point, |to_hash| hasher.hash_3d([to_hash[0], to_hash[1], variant]))
}

#[inline(always)]
pub fn simplex_3d(point: Vector3<f64>, hasher: &PermutationTable) -> f64 {
    base_simplex_3d(point, |to_hash| hasher.hash_3d(to_hash)).0
}

#[inline(always)]
pub fn simplex_3d_deriv(point: Vector3<f64>, hasher: &PermutationTable) -> (f64, Vector3<f64>) {
    base_simplex_3d(point, |to_hash| hasher.hash_3d(to_hash))
}

#[inline(always)]
pub fn simplex_3d_variant(point: Vector3<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_simplex_3d(point, |to_hash| hasher.hash_4d([to_hash[0], to_hash[1], to_hash[2], variant])).0
}

#[inline(always)]
pub fn simplex_3d_variant_deriv(point: Vector3<f64>, variant: isize, hasher: &PermutationTable) -> (f64, Vector3<f64>) {
    base_simplex_3d(point, |to_hash| hasher.hash_4d([to_hash[0], to_hash[1], to_hash[2], variant]))
}

#[inline(always)]
pub fn simplex_4d(point: Vector4<f64>, hasher: &PermutationTable) -> f64 {
    base_simplex_4d(point, |to_hash| hasher.hash_4d(to_hash)).0
}

#[inline(always)]
pub fn simplex_4d_deriv(point: Vector4<f64>, hasher: &PermutationTable) -> (f64, Vector4<f64>) {
    base_simplex_4d(point, |to_hash| hasher.hash_4d(to_hash))
}

#[inline(always)]
pub fn simplex_4d_variant(point: Vector4<f64>, variant: isize, hasher: &PermutationTable) -> f64 {
    base_simplex_4d(point, |to_hash| hasher.hash_5d([to_hash[0], to_hash[1], to_hash[2], to_hash[3], variant])).0
}

#[inline(always)]
pub fn simplex_4d_variant_deriv(point: Vector4<f64>, variant: isize, hasher: &PermutationTable) -> (f64, Vector4<f64>) {
    base_simplex_4d(point, |to_hash| hasher.hash_5d([to_hash[0], to_hash[1], to_hash[2], to_hash[3], variant]))
}
