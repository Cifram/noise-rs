pub fn fbm_2d<F>(point: [f64; 2], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 2], usize) -> f64
{
    let [mut x, mut y] = point;
    x *= frequency;
    y *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        scale *= persistence;
    }
    result
}

pub fn fbm_3d<F>(point: [f64; 3], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 3], usize) -> f64
{
    let [mut x, mut y, mut z] = point;
    x *= frequency;
    y *= frequency;
    z *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y, z], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        scale *= persistence;
    }
    result
}

pub fn fbm_4d<F>(point: [f64; 4], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 4], usize) -> f64
{
    let [mut x, mut y, mut z, mut w] = point;
    x *= frequency;
    y *= frequency;
    z *= frequency;
    w *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y, z, w], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        w *= lacunarity;
        scale *= persistence;
    }
    result
}
