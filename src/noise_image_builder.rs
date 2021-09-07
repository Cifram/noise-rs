use std::{fs, path::Path};

pub struct NoiseImageBuilder<SourceFn, const DIM: usize>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    x_bounds: (f64, f64),
    y_bounds: (f64, f64),
    size: (usize, usize),
    source_fn: SourceFn,
    gradient: Option<ColorGradient>,
}

impl<SourceFn, const DIM: usize> NoiseImageBuilder<SourceFn, DIM>
where
    SourceFn: Fn([f64; DIM]) -> f64,
{
    pub fn new(source_fn: SourceFn) -> Self {
        NoiseImageBuilder {
            x_bounds: (-1.0, 1.0),
            y_bounds: (-1.0, 1.0),
            size: (100, 100),
            source_fn,
            gradient: None,
        }
    }

    pub fn set_x_bounds(self, lower_x_bound: f64, upper_x_bound: f64) -> Self {
        NoiseImageBuilder {
            x_bounds: (lower_x_bound, upper_x_bound),
            ..self
        }
    }

    pub fn set_y_bounds(self, lower_y_bound: f64, upper_y_bound: f64) -> Self {
        NoiseImageBuilder {
            y_bounds: (lower_y_bound, upper_y_bound),
            ..self
        }
    }

    pub fn set_size(self, width: usize, height: usize) -> Self {
        NoiseImageBuilder {
            size: (width, height),
            ..self
        }
    }

    pub fn set_gradient(self, gradient: ColorGradient) -> Self {
        NoiseImageBuilder {
            gradient: Some(gradient),
            ..self
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        // Create the output directory for the images, if it doesn't already exist
        let target_dir = Path::new("example_images/");

        if !target_dir.exists() {
            fs::create_dir(target_dir).expect("failed to create example_images directory");
        }

        //concatenate the directory to the filename string
        let directory: String = "example_images/".to_owned();
        let file_path = directory + filename;

        let (width, height) = self.size;

        let x_extent = self.x_bounds.1 - self.x_bounds.0;
        let y_extent = self.y_bounds.1 - self.y_bounds.0;

        let x_step = x_extent / width as f64;
        let y_step = y_extent / height as f64;

        if let Some(gradient) = &self.gradient {
            let mut pixels: Vec<u8> = vec![0; width*height*4];
            for y in 0..height {
                let current_y = self.y_bounds.0 + y_step * y as f64;
                for x in 0..width {
                    let current_x = self.x_bounds.0 + x_step * x as f64;
                    let noise_value = (self.source_fn)(pad_array(&[current_x, current_y]));
                    let color = gradient.get_color(noise_value);
                    let index = (y*width + x)*4;
                    pixels[index] = color[0];
                    pixels[index+1] = color[1];
                    pixels[index+2] = color[2];
                    pixels[index+3] = color[3];
                }
            }

            let _ = image::save_buffer(
                &Path::new(&file_path),
                &*pixels,
                self.size.0 as u32,
                self.size.1 as u32,
                image::ColorType::Rgba8,
            );
        } else {
            let mut pixels: Vec<u8> = vec![0; width*height];
            for y in 0..height {
                let current_y = self.y_bounds.0 + y_step * y as f64;
                for x in 0..width {
                    let current_x = self.x_bounds.0 + x_step * x as f64;
                    let noise_value = (self.source_fn)(pad_array(&[current_x, current_y]));
                    pixels[y*width + x] = ((noise_value * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
                }
            }

            let _ = image::save_buffer(
                &Path::new(&file_path),
                &*pixels,
                self.size.0 as u32,
                self.size.1 as u32,
                image::ColorType::L8,
            );
        }

        println!("\nFinished generating {}", filename);
    }
}

pub type Color = [u8; 4];

#[derive(Clone, Copy, Debug, Default)]
struct GradientPoint {
    pos: f64,
    color: Color,
}

#[derive(Clone, Copy, Debug, Default)]
struct GradientDomain {
    min: f64,
    max: f64,
}

impl GradientDomain {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn set_min(&mut self, min: f64) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: f64) {
        self.max = max;
    }
}

#[derive(Clone, Debug, Default)]
pub struct ColorGradient {
    gradient_points: Vec<GradientPoint>,
    domain: GradientDomain,
}

impl ColorGradient {
    pub fn new() -> Self {
        let gradient = Self {
            gradient_points: Vec::new(),
            domain: GradientDomain::new(0.0, 1.0),
        };

        gradient.build_grayscale_gradient()
    }

    pub fn add_gradient_point(mut self, pos: f64, color: Color) -> Self {
        let new_point = GradientPoint { pos, color };

        // first check to see if the position is within the domain of the gradient. if the position
        // is not within the domain, expand the domain and add the GradientPoint
        if self.domain.min > pos {
            self.domain.set_min(pos);
            // since the new position is the smallest value, insert it at the beginning of the
            // gradient
            self.gradient_points.insert(0, new_point);
        } else if self.domain.max < pos {
            self.domain.set_max(pos);

            // since the new position is at the largest value, insert it at the end of the gradient
            self.gradient_points.push(new_point)
        } else if !self // new point must be somewhere inside the existing domain. Check to see if
            // it doesn't exist already
            .gradient_points
            .iter()
            .any(|&x| (x.pos - pos).abs() < f64::EPSILON)
        {
            // it doesn't, so find the correct position to insert the new
            // control point.
            let insertion_point = self.find_insertion_point(pos);

            // add the new control point at the correct position.
            self.gradient_points.insert(insertion_point, new_point);
        }

        self
    }

    fn find_insertion_point(&self, pos: f64) -> usize {
        self.gradient_points
            .iter()
            .position(|x| x.pos >= pos)
            .unwrap_or_else(|| self.gradient_points.len())
    }

    pub fn clear_gradient(mut self) -> Self {
        self.gradient_points.clear();
        self.domain = GradientDomain::new(0.0, 0.0);

        self
    }

    pub fn build_grayscale_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.0, [0, 0, 0, 255])
            .add_gradient_point(1.0, [255, 255, 255, 255])
    }

    #[rustfmt::skip]
    pub fn build_terrain_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.00,              [  0,   0,   0, 255])
            .add_gradient_point(-256.0 / 16384.0,   [  6,  58, 127, 255])
            .add_gradient_point(-1.0 / 16384.0,     [ 14, 112, 192, 255])
            .add_gradient_point(0.0,                [ 70, 120,  60, 255])
            .add_gradient_point(1024.0 / 16384.0,   [110, 140,  75, 255])
            .add_gradient_point(2048.0 / 16384.0,   [160, 140, 111, 255])
            .add_gradient_point(3072.0 / 16384.0,   [184, 163, 141, 255])
            .add_gradient_point(4096.0 / 16384.0,   [128, 128, 128, 255])
            .add_gradient_point(5632.0 / 16384.0,   [128, 128, 128, 255])
            .add_gradient_point(6144.0 / 16384.0,   [250, 250, 250, 255])
            .add_gradient_point(1.0,                [255, 255, 255, 255])
    }

    #[rustfmt::skip]
    pub fn build_rainbow_gradient(self) -> Self {
        self.clear_gradient()
            .add_gradient_point(-1.0, [255,   0,   0, 255])
            .add_gradient_point(-0.7, [255, 255,   0, 255])
            .add_gradient_point(-0.4, [  0, 255,   0, 255])
            .add_gradient_point( 0.0, [  0, 255, 255, 255])
            .add_gradient_point( 0.3, [  0,   0, 255, 255])
            .add_gradient_point( 0.6, [255,   0, 255, 255])
            .add_gradient_point( 1.0, [255,   0,   0, 255])
    }

    pub fn get_color(&self, pos: f64) -> Color {
        let mut color = Color::default();

        // If there are no colors in the gradient, return black
        if !self.gradient_points.is_empty() {
            match () {
                _ if pos < self.domain.min => color = self.gradient_points.first().unwrap().color,
                _ if pos > self.domain.max => color = self.gradient_points.last().unwrap().color,
                _ => {
                    for points in self.gradient_points.windows(2) {
                        if (points[0].pos <= pos) && (points[1].pos > pos) {
                            // Compute the alpha value used for linear interpolation
                            let alpha = (pos - points[0].pos) / (points[1].pos - points[0].pos);

                            // Now perform the interpolation and return.
                            color = interpolate_color(points[0].color, points[1].color, alpha)
                        }
                    }
                }
            };
        };

        color
    }
}

fn interpolate_color(color0: Color, color1: Color, alpha: f64) -> Color {
    fn blend_channel(a: u8, b: u8, alpha: f64) -> u8 {
        let a = a as f64;
        let b = b as f64;
        (b * alpha + a * (1.0 - alpha)) as u8
    }
    [
        blend_channel(color0[0], color1[0], alpha),
        blend_channel(color0[1], color1[1], alpha),
        blend_channel(color0[2], color1[2], alpha),
        blend_channel(color0[3], color1[3], alpha),
    ]
}

fn pad_array<const SIZE: usize>(values: &[f64]) -> [f64; SIZE] {
    let mut result = [0.0; SIZE];
    for i in 0..values.len().min(SIZE) {
        result[i] = values[i];
    }
    result
}
