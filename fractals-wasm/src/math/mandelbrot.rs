use math::{FractalGenerator, LinearFractalGenerator, FractalConfig};
use math::FractalPoint;
use math::complex::ComplexF64;

pub struct MandelbrotGenerator;

impl FractalGenerator for MandelbrotGenerator {
    fn generate(&self, conf: &FractalConfig) -> Vec<FractalPoint> {
        LinearFractalGenerator::generate(self, conf)
    }
}

impl LinearFractalGenerator for MandelbrotGenerator {

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
            x += c.re();
            y += c.im();
        }
    }
}