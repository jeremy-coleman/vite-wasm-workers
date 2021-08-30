use math::{FractalGenerator, FractalConfig, FractalPoint, LinearFractalGenerator};
use math::complex::ComplexF64;

pub struct JuliaGenerator {
    p_re: f64,
    p_im: f64,
}

impl FractalGenerator for JuliaGenerator {
    fn generate(&self, conf: &FractalConfig) -> Vec<FractalPoint> {
        LinearFractalGenerator::generate(self, conf)
    }
}

impl JuliaGenerator {
    pub fn new(p_re: f64, p_im: f64) -> Self {
        JuliaGenerator {
            p_re,
            p_im
        }
    }
}

impl LinearFractalGenerator for JuliaGenerator {

    fn convergence(&self, c: ComplexF64, max_iters: usize) -> FractalPoint {
        let mut x = c.re();
        let mut y = c.im();

        let mut i = 0;

        loop {

            let x2 = x * x;
            let y2 = y * y;

            if x2 + y2 > 4.0 || i >= max_iters {
                return FractalPoint {
                    iterations: i,
                    re: x,
                    im: y
                }
            }

            let xy = x * y;

            x = x2 - y2;
            y = 2.0 * xy;
            i += 1;
            x += self.p_re;
            y += self.p_im;
        }
    }
}