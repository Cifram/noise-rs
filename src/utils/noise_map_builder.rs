use crate::{math::interpolate, utils::noise_map::NoiseMap};

fn pad_array<const SIZE: usize>(values: &[f64]) -> [f64; SIZE] {
    let mut result = [0.0; SIZE];
    for i in 0..values.len().min(SIZE) {
        result[i] = values[i];
    }
    result
}

pub trait NoiseMapBuilder<SourceFn> {
    fn set_size(self, width: usize, height: usize) -> Self;

    fn set_source_module(self, source_fn: SourceFn) -> Self;

    fn size(&self) -> (usize, usize);

    fn build(&self) -> NoiseMap;
}

pub struct PlaneMapBuilder<SourceFn, const DIM: usize>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    is_seamless: bool,
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    size: (usize, usize),
    source_fn: SourceFn,
}

impl<SourceFn, const DIM: usize> PlaneMapBuilder<SourceFn, DIM>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    pub fn new_fn(func: SourceFn) -> Self {
        PlaneMapBuilder {
            is_seamless: false,
            x_bounds: (-1.0, 1.0),
            y_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_fn: func,
        }
    }
}

impl<SourceFn> PlaneMapBuilder<SourceFn, 2>
where
    SourceFn: Fn([f64; 2]) -> f64,
{
    pub fn new_2d(source_fn: SourceFn) -> Self {
        Self::new(source_fn)
    }
}

impl<SourceFn> PlaneMapBuilder<SourceFn, 3>
where
    SourceFn: Fn([f64; 3]) -> f64,
{
    pub fn new_3d(source_fn: SourceFn) -> Self {
        Self::new(source_fn)
    }
}

impl<SourceFn> PlaneMapBuilder<SourceFn, 4>
where
    SourceFn: Fn([f64; 4]) -> f64,
{
    pub fn new_4d(source_fn: SourceFn) -> Self {
        Self::new(source_fn)
    }
}

impl<SourceFn, const DIM: usize> PlaneMapBuilder<SourceFn, DIM>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    pub fn new(source_fn: SourceFn) -> Self {
        PlaneMapBuilder {
            is_seamless: false,
            x_bounds: (-1.0, 1.0),
            y_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_fn,
        }
    }

    pub fn set_is_seamless(self, is_seamless: bool) -> Self {
        PlaneMapBuilder {
            is_seamless,
            ..self
        }
    }

    pub fn set_x_bounds(self, lower_x_bound: f64, upper_x_bound: f64) -> Self {
        PlaneMapBuilder {
            x_bounds: (lower_x_bound, upper_x_bound),
            ..self
        }
    }

    pub fn set_y_bounds(self, lower_y_bound: f64, upper_y_bound: f64) -> Self {
        PlaneMapBuilder {
            y_bounds: (lower_y_bound, upper_y_bound),
            ..self
        }
    }

    pub fn x_bounds(&self) -> (f64, f64) {
        self.x_bounds
    }

    pub fn y_bounds(&self) -> (f64, f64) {
        self.y_bounds
    }
}

impl<SourceFn, const DIM: usize> NoiseMapBuilder<SourceFn> for PlaneMapBuilder<SourceFn, DIM>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    fn set_size(self, width: usize, height: usize) -> Self {
        PlaneMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_fn: SourceFn) -> Self {
        PlaneMapBuilder {
            source_fn,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let (width, height) = self.size;

        let mut result_map = NoiseMap::new(width, height);

        let x_extent = self.x_bounds.1 - self.x_bounds.0;
        let y_extent = self.y_bounds.1 - self.y_bounds.0;

        let x_step = x_extent / width as f64;
        let y_step = y_extent / height as f64;

        for y in 0..height {
            let current_y = self.y_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_x = self.x_bounds.0 + x_step * x as f64;

                let final_value = if self.is_seamless {
                    let sw_value = (self.source_fn)(pad_array(&[current_x, current_y]));
                    let se_value = (self.source_fn)(pad_array(&[current_x + x_extent, current_y]));
                    let nw_value = (self.source_fn)(pad_array(&[current_x, current_y + y_extent]));
                    let ne_value = (self.source_fn)(pad_array(&[current_x + x_extent, current_y + y_extent]));

                    let x_blend = 1.0 - ((current_x - self.x_bounds.0) / x_extent);
                    let y_blend = 1.0 - ((current_y - self.y_bounds.0) / y_extent);

                    let y0 = interpolate::linear(sw_value, se_value, x_blend);
                    let y1 = interpolate::linear(nw_value, ne_value, x_blend);

                    interpolate::linear(y0, y1, y_blend)
                } else {
                    (self.source_fn)(pad_array(&[current_x, current_y]))
                };

                result_map[(x, y)] = final_value;
            }
        }

        result_map
    }
}

pub struct SphereMapBuilder<SourceFn>
where
    SourceFn: Fn([f64; 3]) -> f64,
{
    latitude_bounds: (f64, f64),
    longitude_bounds: (f64, f64),
    size: (usize, usize),
    source_fn: SourceFn,
}

impl<SourceFn> SphereMapBuilder<SourceFn>
where
    SourceFn: Fn([f64; 3]) -> f64,
{
    pub fn new(source_fn: SourceFn) -> Self {
        SphereMapBuilder {
            latitude_bounds: (-1.0, 1.0),
            longitude_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_fn,
        }
    }

    pub fn set_latitude_bounds(self, min_lat_bound: f64, max_lat_bound: f64) -> Self {
        SphereMapBuilder {
            latitude_bounds: (min_lat_bound, max_lat_bound),
            ..self
        }
    }

    pub fn set_longitude_bounds(self, min_lon_bound: f64, max_lon_bound: f64) -> Self {
        SphereMapBuilder {
            longitude_bounds: (min_lon_bound, max_lon_bound),
            ..self
        }
    }

    pub fn set_bounds(
        self,
        min_lat_bound: f64,
        max_lat_bound: f64,
        min_lon_bound: f64,
        max_lon_bound: f64,
    ) -> Self {
        SphereMapBuilder {
            latitude_bounds: (min_lat_bound, max_lat_bound),
            longitude_bounds: (min_lon_bound, max_lon_bound),
            ..self
        }
    }

    pub fn latitude_bounds(&self) -> (f64, f64) {
        self.latitude_bounds
    }

    pub fn longitude_bounds(&self) -> (f64, f64) {
        self.longitude_bounds
    }
}

impl<SourceFn> NoiseMapBuilder<SourceFn> for SphereMapBuilder<SourceFn>
where
    SourceFn: Fn([f64; 3]) -> f64,
{
    fn set_size(self, width: usize, height: usize) -> Self {
        SphereMapBuilder {
            size: (width, height),
            ..self
        }
    }

    fn set_source_module(self, source_fn: SourceFn) -> Self {
        SphereMapBuilder {
            source_fn,
            ..self
        }
    }

    fn size(&self) -> (usize, usize) {
        self.size
    }

    fn build(&self) -> NoiseMap {
        let (width, height) = self.size;

        let mut result_map = NoiseMap::new(width, height);

        let lon_extent = self.longitude_bounds.1 - self.longitude_bounds.0;
        let lat_extent = self.latitude_bounds.1 - self.latitude_bounds.0;

        let x_step = lon_extent / width as f64;
        let y_step = lat_extent / height as f64;

        for y in 0..height {
            let current_lat = self.latitude_bounds.0 + y_step * y as f64;

            for x in 0..width {
                let current_lon = self.longitude_bounds.0 + x_step * x as f64;

                let point = lat_lon_to_xyz(current_lat, current_lon);

                result_map[(x, y)] = (self.source_fn)(point);
            }
        }

        result_map
    }
}

fn lat_lon_to_xyz(lat: f64, lon: f64) -> [f64; 3] {
    let r = lat.to_radians().cos();
    let x = r * lon.to_radians().cos();
    let y = lat.to_radians().sin();
    let z = r * lon.to_radians().sin();

    [x, y, z]
}
