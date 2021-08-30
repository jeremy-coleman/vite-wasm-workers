use math::FractalPoint;
use palette::{Hsv, rgb::Srgb};

pub trait ColorTransformation {

    fn determine_color_hsv(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32);

    fn determine_color(&self, point: &FractalPoint, max_iters: usize) -> (u8, u8, u8) {
        return hsv_to_rgb(self.determine_color_hsv(point, max_iters));
    }

    fn transform(
        &self,
        points: Vec<FractalPoint>,
        max_iters: usize,
        width: usize,
        height: usize,
        scaling: usize
    ) -> Vec<u8> {
        let mut image_data = Vec::with_capacity(4 * scaling * scaling * points.len());

        for y in 0..height {

            for _ in 0..scaling {

                for x in 0..width {

                    let point_index = width * y + x;

                    let (r, g, b) = self.determine_color(
                        points.get(point_index).expect("Point out of bounds!"),
                        max_iters
                    );

                    for _ in 0..scaling {
                        image_data.push(r);
                        image_data.push(g);
                        image_data.push(b);
                        image_data.push(255);
                    }
                }
            }
        }

        return image_data;
    }
}

pub enum HsvTransMode {
    Hue, Saturation, Value
}

fn hsv_to_rgb(hsv: (f32, f32, f32)) -> (u8, u8, u8) {
    let (h, s, v) = hsv;
    let color_hsv = Hsv::new(h, s, v);
    let color_rgb = Srgb::from(color_hsv);

    return (
        (color_rgb.red * 255.0) as u8,
        (color_rgb.green * 255.0) as u8,
        (color_rgb.blue * 255.0) as u8
    )
}

pub struct HsvBasedColorTransformation {
    pub h_base: f32,
    pub s_base: f32,
    pub v_base: f32,
    pub mode: HsvTransMode
}

impl HsvBasedColorTransformation {
    fn hue_based(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32) {
        let iterations = point.iterations << 2;
        let modifier = iterations as f32 / 1000.0;

        return (modifier * self.h_base, self.s_base, self.v_base)
    }

    fn saturation_based(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32) {
        let iterations = point.iterations << 2;
        let modifier = iterations as f32 / 1000.0;

        return (self.h_base, modifier * self.s_base, self.v_base)
    }

    fn value_based(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32) {
        let iterations = point.iterations << 2;
        let modifier = iterations as f32 / 1000.0;

        return (self.h_base, self.s_base, modifier * self.v_base)
    }
}

impl ColorTransformation for HsvBasedColorTransformation {

    fn determine_color_hsv(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32) {

        return if point.iterations == max_iters {
            (0.0, 0.0, 0.0)
        } else {
            match self.mode {
                HsvTransMode::Hue => self.hue_based(point, max_iters),
                HsvTransMode::Saturation => self.saturation_based(point, max_iters),
                HsvTransMode::Value => self.value_based(point, max_iters)
            }
        }
    }


    fn determine_color(&self, point: &FractalPoint, max_iters: usize) -> (u8, u8, u8) {
        return hsv_to_rgb(self.determine_color_hsv(point, max_iters))
    }
}

pub struct SmoothColorTransformation<T : ColorTransformation> {
    pub base: T
}

#[inline(always)]
fn in_between(a: f32, b: f32, factor: f32) -> f32 {
    return a + (b - a) * factor;
}

#[inline(always)]
fn interpolate(color: (f32, f32, f32), color2: (f32, f32, f32), value: f32) -> (f32, f32, f32) {
    return (
        in_between(color.0, color2.0, value),
        in_between(color.1, color2.1, value),
        in_between(color.2, color2.2, value)
    )
}

impl<T : ColorTransformation> ColorTransformation for SmoothColorTransformation<T> {
    fn determine_color_hsv(&self, point: &FractalPoint, max_iters: usize) -> (f32, f32, f32) {

        if point.iterations >= max_iters {
            return self.base.determine_color_hsv(point, max_iters);
        }

        let log_zn = ((point.re * point.re + point.im * point.im).ln() / 2.0) as f32;
        let log_2 = 2.0_f32.ln();
        let nu = (log_zn / log_2).ln() / log_2;
        let iter = (point.iterations as f32) + 1.0 - nu;

        let new_iters = iter.floor();

        let point1 = FractalPoint {
            re: point.re,
            im: point.im,
            iterations: new_iters as usize
        };

        let point2 = FractalPoint {
            re: point.re,
            im: point.im,
            iterations: new_iters as usize + 1
        };

        let color1 = self.base.determine_color_hsv(&point1, max_iters);
        let color2 = self.base.determine_color_hsv(&point2, max_iters);

        return interpolate(color1, color2, iter % 1.0);
    }
}