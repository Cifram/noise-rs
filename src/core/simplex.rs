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

    // Using the Vector3 struct for all the internal logic causes a significant
    // performance regression, for some reason, so we instead break it out into
    // it's component elements and do all the calculations directly on those.
    let (x, y, z) = point.into();

    /* Skew the input space to determine which simplex cell we're in */
    // let skew = (x + y + z) * SKEW_FACTOR_3D; /* Very nice and simple skew factor for 3D */
    let skew = (x + y + z) * SKEW_FACTOR_3D;
    let skewedx = x + skew;
    let skewedy = y + skew;
    let skewedz = z + skew;
    let floorx = skewedx.floor();
    let floory = skewedy.floor();
    let floorz = skewedz.floor();
    let cellx = floorx as isize;
    let celly = floory as isize;
    let cellz = floorz as isize;

    // let unskew = (cell_x + cell_y + cell_z) as f64 * UNSKEW_FACTOR_3D;
    let unskew = (floorx + floory + floorz) * UNSKEW_FACTOR_3D;
    /* Unskew the cell origin back to (x,y,z) space */
    let unskewedx = floorx - unskew;
    let unskewedy = floory - unskew;
    let unskewedz = floorz - unskew;
    /* The x,y,z distances from the cell origin */
    let distance1x = x - unskewedx;
    let distance1y = y - unskewedy;
    let distance1z = z - unskewedz;

    /* For the 3D case, the simplex shape is a slightly irregular tetrahedron.
     * Determine which simplex we are in. */
    /* TODO: This code would benefit from a backport from the GLSL version! */
    let (order1x, order1y, order1z, order2x, order2y, order2z) = if distance1x >= distance1y {
        if distance1y >= distance1z {
            /* X Y Z order */
            (1, 0, 0, 1, 1, 0)
        } else if distance1x >= distance1z {
            /* X Z Y order */
            (1, 0, 0, 1, 0, 1)
        } else {
            /* Z X Y order */
            (0, 0, 1, 1, 0, 1)
        }
    } else {
        // x0<y0
        if distance1y < distance1z {
            /* Z Y X order */
            (0, 0, 1, 0, 1, 1)
        } else if distance1x < distance1z {
            /* Y Z X order */
            (0, 1, 0, 0, 1, 1)
        } else {
            /* Y X Z order */
            (0, 1, 0, 1, 1, 0)
        }
    };

    /* A step of (1,0,0) in (i,j,k) means a step of (1-c,-c,-c) in (x,y,z),
     * a step of (0,1,0) in (i,j,k) means a step of (-c,1-c,-c) in (x,y,z), and
     * a step of (0,0,1) in (i,j,k) means a step of (-c,-c,1-c) in (x,y,z), where
     * c = 1/6.   */

    let distance2x = distance1x - order1x as f64 + UNSKEW_FACTOR_3D;
    let distance2y = distance1y - order1y as f64 + UNSKEW_FACTOR_3D;
    let distance2z = distance1z - order1z as f64 + UNSKEW_FACTOR_3D;
    let distance3x = distance1x - order2x as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let distance3y = distance1y - order2y as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let distance3z = distance1z - order2z as f64 + 2.0 * UNSKEW_FACTOR_3D;
    let distance4x = distance1x - 1.0 + 3.0 * UNSKEW_FACTOR_3D;
    let distance4y = distance1y - 1.0 + 3.0 * UNSKEW_FACTOR_3D;
    let distance4z = distance1z - 1.0 + 3.0 * UNSKEW_FACTOR_3D;

    // Calculate gradient indexes for each corner
    let gi1 = hasher([cellx, celly, cellz]);
    let gi2 = hasher([cellx + order1x, celly + order1y, cellz + order1z]);
    let gi3 = hasher([cellx + order2x, celly + order2y, cellz + order2z]);
    let gi4 = hasher([cellx + 1, celly + 1, cellz + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradx: f64,
        grady: f64,
        gradz: f64,
    }

    fn surflet(grad_index: usize, x: f64, y: f64, z: f64) -> SurfletComponents {
        let t = 0.5 - (x*x + y*y + z*z);

        if t > 0.0 {
            let [gradx, grady, gradz] = gradient::grad3(grad_index);
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * (gradx*x + grady*y + gradz*z),
                t, t2, t4,
                gradx, grady, gradz
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0,
                t2: 0.0,
                t4: 0.0,
                gradx: 0.0,
                grady: 0.0,
                gradz: 0.0,
            }
        }
    }

    /* Calculate the contribution from the four corners */
    let corner1 = surflet(gi1, distance1x, distance1y, distance1z);
    let corner2 = surflet(gi2, distance2x, distance2y, distance2z);
    let corner3 = surflet(gi3, distance3x, distance3y, distance3z);
    let corner4 = surflet(gi4, distance4x, distance4y, distance4z);

    /*  Add contributions from each corner to get the final noise value.
     * The result is scaled to return values in the range [-1,1] */
    let noise = 28.0 * (corner1.value + corner2.value + corner3.value + corner4.value);

    /*  A straight, unoptimised calculation would be like:
     *    dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gx0;
     *    dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gy0;
     *    dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, x0, y0, z0) + t40 * gz0;
     *    dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gx1;
     *    dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gy1;
     *    dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, x1, y1, z1) + t41 * gz1;
     *    dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gx2;
     *    dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gy2;
     *    dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, x2, y2, z2) + t42 * gz2;
     *    dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gx3;
     *    dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gy3;
     *    dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, x3, y3, z3) + t43 * gz3;
     */
    let temp0 = corner1.t2 * corner1.t * (corner1.gradx*distance1x + corner1.grady*distance1y + corner1.gradz*distance1z);
    let mut dnoisex = distance1x * temp0;
    let mut dnoisey = distance1y * temp0;
    let mut dnoisez = distance1z * temp0;

    let temp1 = corner2.t2 * corner2.t * (corner2.gradx*distance2x + corner2.grady*distance2y + corner2.gradz*distance2z);
    dnoisex += distance2x * temp1;
    dnoisey += distance2y * temp1;
    dnoisez += distance2z * temp1;

    let temp2 = corner3.t2 * corner3.t * (corner3.gradx*distance3x + corner3.grady*distance3y + corner3.gradz*distance3z);
    dnoisex += distance3x * temp2;
    dnoisey += distance3y * temp2;
    dnoisez += distance3z * temp2;

    let temp3 = corner4.t2 * corner4.t * (corner4.gradx*distance4x + corner4.grady*distance4y + corner4.gradz*distance4z);
    dnoisex += distance4x * temp3;
    dnoisey += distance4y * temp3;
    dnoisez += distance4z * temp3;

    dnoisex *= -8.0;
    dnoisey *= -8.0;
    dnoisez *= -8.0;

    dnoisex +=
        corner1.gradx * corner1.t4 +
        corner2.gradx * corner2.t4 +
        corner3.gradx * corner3.t4 +
        corner4.gradx * corner4.t4;
    dnoisey +=
        corner1.grady * corner1.t4 +
        corner2.grady * corner2.t4 +
        corner3.grady * corner3.t4 +
        corner4.grady * corner4.t4;
    dnoisez +=
        corner1.gradz * corner1.t4 +
        corner2.gradz * corner2.t4 +
        corner3.gradz * corner3.t4 +
        corner4.gradz * corner4.t4;

    /* Scale derivative to match the noise scaling */
    dnoisex *= 28.0;
    dnoisey *= 28.0;
    dnoisez *= 28.0;

    (noise, Vector3::new(dnoisex, dnoisey, dnoisez))
}

#[inline(always)]
fn base_simplex_4d<F>(point: Vector4<f64>, hasher: F) -> (f64, Vector4<f64>)
where
    F: Fn([isize; 4]) -> usize
{
    const SKEW_FACTOR_4D: f64 = 0.309016994;
    const UNSKEW_FACTOR_4D: f64 = 0.138196601;

    // Using the Vector4 struct for all the internal logic causes a significant
    // performance regression, for some reason, so we instead break it out into
    // it's component elements and do all the calculations directly on those.
    let (x, y, z, w) = point.into();

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    // Factor for 4D skewing
    let skew = (x + y + z + w) * SKEW_FACTOR_4D;
    let skewedx = x + skew;
    let skewedy = y + skew;
    let skewedz = z + skew;
    let skewedw = w + skew;
    let floorx = skewedx.floor();
    let floory = skewedy.floor();
    let floorz = skewedz.floor();
    let floorw = skewedw.floor();
    let cellx = floorx as isize;
    let celly = floory as isize;
    let cellz = floorz as isize;
    let cellw = floorw as isize;

    // Factor for 4D unskewing
    let unskew = (floorx + floory + floorz + floorw) * UNSKEW_FACTOR_4D;
    // Unskew the cell origin back to (x,y,z,w) space
    let unskewedx = floorx - unskew;
    let unskewedy = floory - unskew;
    let unskewedz = floorz - unskew;
    let unskewedw = floorw - unskew;

    // The x,y,z,w distances from the cell origin
    let distance1x = x - unskewedx;
    let distance1y = y - unskewedy;
    let distance1z = z - unskewedz;
    let distance1w = w - unskewedw;

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // The method below is a reasonable way of finding the ordering of x,y,z,w
    // and then find the correct traversal order for the simplex we're in.
    // First, six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and then the results are used to add up binary
    // bits for an integer index into a precomputed lookup table, simplex[].
    let c1 = if distance1x > distance1y { 32 } else { 0 };
    let c2 = if distance1x > distance1z { 16 } else { 0 };
    let c3 = if distance1y > distance1z { 8 } else { 0 };
    let c4 = if distance1x > distance1w { 4 } else { 0 };
    let c5 = if distance1y > distance1w { 2 } else { 0 };
    let c6 = if distance1z > distance1w { 1 } else { 0 };
    let c = c1 | c2 | c3 | c4 | c5 | c6; // '|' is mostly faster than '+'

    // These are the integer offsets of the other 3 simplex corners.
    // simplex[c] is a 4-vector with the numbers 0, 1, 2 and 3 in some order.
    // Many values of c will never occur, since e.g. x>y>z>w makes x<z, y<w and x<w
    // impossible. Only the 24 indices which have non-zero entries make any sense.
    // We use a thresholding to set the coordinates in turn from the largest magnitude.
    // The number 3 in the "simplex" array is at the position of the largest coordinate.
    let order1x = if SIMPLEX[c][0] >= 3 { 1 } else { 0 };
    let order1y = if SIMPLEX[c][1] >= 3 { 1 } else { 0 };
    let order1z = if SIMPLEX[c][2] >= 3 { 1 } else { 0 };
    let order1w = if SIMPLEX[c][3] >= 3 { 1 } else { 0 };
    // The number 2 in the "simplex" array is at the second largest coordinate.
    let order2x = if SIMPLEX[c][0] >= 2 { 1 } else { 0 };
    let order2y = if SIMPLEX[c][1] >= 2 { 1 } else { 0 };
    let order2z = if SIMPLEX[c][2] >= 2 { 1 } else { 0 };
    let order2w = if SIMPLEX[c][3] >= 2 { 1 } else { 0 };
    // The number 1 in the "simplex" array is at the second smallest coordinate.
    let order3x = if SIMPLEX[c][0] >= 1 { 1 } else { 0 };
    let order3y = if SIMPLEX[c][1] >= 1 { 1 } else { 0 };
    let order3z = if SIMPLEX[c][2] >= 1 { 1 } else { 0 };
    let order3w = if SIMPLEX[c][3] >= 1 { 1 } else { 0 };
    // The fifth corner has all coordinate offsets = 1, so no need to look that up.

    // Offsets for second corner in (x,y,z,w) coords
    let distance2x = distance1x - order1x as f64 + UNSKEW_FACTOR_4D;
    let distance2y = distance1y - order1y as f64 + UNSKEW_FACTOR_4D;
    let distance2z = distance1z - order1z as f64 + UNSKEW_FACTOR_4D;
    let distance2w = distance1w - order1w as f64 + UNSKEW_FACTOR_4D;
    // Offsets for third corner in (x,y,z,w) coords
    let distance3x = distance1x - order2x as f64 + 2.0 * UNSKEW_FACTOR_4D;
    let distance3y = distance1y - order2y as f64 + 2.0 * UNSKEW_FACTOR_4D;
    let distance3z = distance1z - order2z as f64 + 2.0 * UNSKEW_FACTOR_4D;
    let distance3w = distance1w - order2w as f64 + 2.0 * UNSKEW_FACTOR_4D;
    // Offsets for fourth corner in (x,y,z,w) coords
    let distance4x = distance1x - order3x as f64 + 3.0 * UNSKEW_FACTOR_4D;
    let distance4y = distance1y - order3y as f64 + 3.0 * UNSKEW_FACTOR_4D;
    let distance4z = distance1z - order3z as f64 + 3.0 * UNSKEW_FACTOR_4D;
    let distance4w = distance1w - order3w as f64 + 3.0 * UNSKEW_FACTOR_4D;
    // Offsets for last corner in (x,y,z,w) coords
    let distance5x = distance1x - 1.0 + 4.0 * UNSKEW_FACTOR_4D;
    let distance5y = distance1y - 1.0 + 4.0 * UNSKEW_FACTOR_4D;
    let distance5z = distance1z - 1.0 + 4.0 * UNSKEW_FACTOR_4D;
    let distance5w = distance1w - 1.0 + 4.0 * UNSKEW_FACTOR_4D;

    // Calculate gradient indexes for each corner
    let gi1 = hasher([cellx, celly, cellz, cellw]);
    let gi2 = hasher([cellx + order1x, celly + order1y, cellz + order1z, cellw + order1w]);
    let gi3 = hasher([cellx + order2x, celly + order2y, cellz + order2z, cellw + order2w]);
    let gi4 = hasher([cellx + order3x, celly + order3y, cellz + order3z, cellw + order3w]);
    let gi5 = hasher([cellx + 1, celly + 1, cellz + 1, cellw + 1]);

    struct SurfletComponents {
        value: f64,
        t: f64,
        t2: f64,
        t4: f64,
        gradx: f64,
        grady: f64,
        gradz: f64,
        gradw: f64,
    }

    fn surflet(grad_index: usize, x: f64, y: f64, z: f64, w: f64) -> SurfletComponents {
        let t = 0.6 - (x*x + y*y + z*z + w*w);

        if t > 0.0 {
            let [gradx, grady, gradz, gradw] = gradient::grad4(grad_index);
            let t2 = t * t;
            let t4 = t2 * t2;

            SurfletComponents {
                value: t4 * (gradx*x + grady*y + gradz*z + gradw*w),
                t, t2, t4,
                gradx, grady, gradz, gradw,
            }
        } else {
            // No influence
            SurfletComponents {
                value: 0.0,
                t: 0.0, t2: 0.0, t4: 0.0,
                gradx: 0.0, grady: 0.0, gradz: 0.0, gradw: 0.0,
            }
        }
    }

    /* Calculate the contribution from the five corners */
    let corner1 = surflet(gi1, distance1x, distance1y, distance1z, distance1w);
    let corner2 = surflet(gi2, distance2x, distance2y, distance2z, distance2w);
    let corner3 = surflet(gi3, distance3x, distance3y, distance3z, distance3w);
    let corner4 = surflet(gi4, distance4x, distance4y, distance4z, distance4w);
    let corner5 = surflet(gi5, distance5x, distance5y, distance5z, distance5w);

    // Sum up and scale the result to cover the range [-1,1]
    let noise =
        27.0 * (corner1.value + corner2.value + corner3.value + corner4.value + corner5.value); // TODO: The scale factor is preliminary!

    /*  A straight, unoptimised calculation would be like:
     *    dnoise_dx = -8.0 * t20 * t0 * x0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gx0;
     *    dnoise_dy = -8.0 * t20 * t0 * y0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gy0;
     *    dnoise_dz = -8.0 * t20 * t0 * z0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gz0;
     *    dnoise_dw = -8.0 * t20 * t0 * w0 * dot(gx0, gy0, gz0, gw0, x0, y0, z0, w0) + t40 * gw0;
     *    dnoise_dx += -8.0 * t21 * t1 * x1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gx1;
     *    dnoise_dy += -8.0 * t21 * t1 * y1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gy1;
     *    dnoise_dz += -8.0 * t21 * t1 * z1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gz1;
     *    dnoise_dw += -8.0 * t21 * t1 * w1 * dot(gx1, gy1, gz1, gw1, x1, y1, z1, w1) + t41 * gw1;
     *    dnoise_dx += -8.0 * t22 * t2 * x2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gx2;
     *    dnoise_dy += -8.0 * t22 * t2 * y2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gy2;
     *    dnoise_dz += -8.0 * t22 * t2 * z2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gz2;
     *    dnoise_dw += -8.0 * t22 * t2 * w2 * dot(gx2, gy2, gz2, gw2, x2, y2, z2, w2) + t42 * gw2;
     *    dnoise_dx += -8.0 * t23 * t3 * x3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gx3;
     *    dnoise_dy += -8.0 * t23 * t3 * y3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gy3;
     *    dnoise_dz += -8.0 * t23 * t3 * z3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gz3;
     *    dnoise_dw += -8.0 * t23 * t3 * w3 * dot(gx3, gy3, gz3, gw3, x3, y3, z3, w3) + t43 * gw3;
     *    dnoise_dx += -8.0 * t24 * t4 * x4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gx4;
     *    dnoise_dy += -8.0 * t24 * t4 * y4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gy4;
     *    dnoise_dz += -8.0 * t24 * t4 * z4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gz4;
     *    dnoise_dw += -8.0 * t24 * t4 * w4 * dot(gx4, gy4, gz4, gw4, x4, y4, z4, w4) + t44 * gw4;
     */
    let temp0 = corner1.t2 * corner1.t *
        (corner1.gradx*distance1x + corner1.grady*distance1y + corner1.gradz*distance1z + corner1.gradw*distance1w);
    let mut dnoisex = distance1x * temp0;
    let mut dnoisey = distance1y * temp0;
    let mut dnoisez = distance1z * temp0;
    let mut dnoisew = distance1w * temp0;

    let temp1 = corner2.t2 * corner2.t *
        (corner2.gradx*distance1x + corner2.grady*distance1y + corner2.gradz*distance1z + corner2.gradw*distance1w);
    dnoisex += distance2x * temp1;
    dnoisey += distance2y * temp1;
    dnoisez += distance2z * temp1;
    dnoisew += distance2w * temp1;

    let temp2 = corner3.t2 * corner3.t *
        (corner3.gradx*distance1x + corner3.grady*distance1y + corner3.gradz*distance1z + corner3.gradw*distance1w);
    dnoisex += distance3x * temp2;
    dnoisey += distance3y * temp2;
    dnoisez += distance3z * temp2;
    dnoisew += distance3w * temp2;

    let temp3 = corner4.t2 * corner4.t *
        (corner4.gradx*distance1x + corner4.grady*distance1y + corner4.gradz*distance1z + corner4.gradw*distance1w);
    dnoisex += distance4x * temp3;
    dnoisey += distance4y * temp3;
    dnoisez += distance4z * temp3;
    dnoisew += distance4w * temp3;

    let temp4 = corner5.t2 * corner5.t *
        (corner5.gradx*distance1x + corner5.grady*distance1y + corner5.gradz*distance1z + corner5.gradw*distance1w);
    dnoisex += distance5x * temp4;
    dnoisey += distance5y * temp4;
    dnoisez += distance5z * temp4;
    dnoisew += distance5w * temp4;

    dnoisex *= -8.0;
    dnoisey *= -8.0;
    dnoisez *= -8.0;
    dnoisew *= -8.0;

    dnoisex +=
        corner1.gradx * corner1.t4 +
        corner2.gradx * corner2.t4 +
        corner3.gradx * corner3.t4 +
        corner4.gradx * corner4.t4 +
        corner5.gradx * corner5.t4;
    dnoisey +=
        corner1.grady * corner1.t4 +
        corner2.grady * corner2.t4 +
        corner3.grady * corner3.t4 +
        corner4.grady * corner4.t4 +
        corner5.grady * corner5.t4;
    dnoisez +=
        corner1.gradz * corner1.t4 +
        corner2.gradz * corner2.t4 +
        corner3.gradz * corner3.t4 +
        corner4.gradz * corner4.t4 +
        corner5.gradz * corner5.t4;
    dnoisew +=
        corner1.gradw * corner1.t4 +
        corner2.gradw * corner2.t4 +
        corner3.gradw * corner3.t4 +
        corner4.gradw * corner4.t4 +
        corner5.gradw * corner5.t4;

    // Scale derivative to match the noise scaling
    dnoisex *= 28.0;
    dnoisey *= 28.0;
    dnoisez *= 28.0;
    dnoisew *= 28.0;

    (noise, Vector4::new(dnoisex, dnoisey, dnoisez, dnoisew))
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
