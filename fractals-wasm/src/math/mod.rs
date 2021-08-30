use math::mandelbrot::MandelbrotGenerator;

use math::complex::ComplexF64;
use math::julia::JuliaGenerator;

pub(crate) mod mandelbrot;
pub(crate) mod complex;
pub(crate) mod julia;

#[derive(Copy, Clone)]
pub struct FractalPoint {
    pub iterations: usize,
    pub re: f64,
    pub im: f64
}

pub enum Generator {
    MANDELBROT, JULIA(f64, f64)
}

impl Generator {
    pub fn create(&self) -> Box<dyn FractalGenerator> {
        return match self {
            Generator::MANDELBROT => { Box::new(MandelbrotGenerator {}) }
            Generator::JULIA(x, y) => { Box::new(JuliaGenerator::new(*x, *y)) }
        }
    }
}

pub struct FractalConfig {
    pub start_re: f64,
    pub end_re: f64,
    pub re_steps: usize,

    pub start_im: f64,
    pub end_im: f64,
    pub im_steps: usize,

    pub max_iters: usize
}

pub trait FractalGenerator {

    fn generate(&self, conf: &FractalConfig) -> Vec<FractalPoint>;
}

pub trait LinearFractalGenerator: FractalGenerator {
    fn convergence(&self, c: ComplexF64, max_iters: usize) -> FractalPoint;

    fn generate(&self, conf: &FractalConfig) -> Vec<FractalPoint> {

        let mut result = Vec::with_capacity(conf.re_steps * conf.im_steps);

        let re_particle = (conf.end_re - conf.start_re) / conf.re_steps as f64;
        let im_particle = (conf.end_im - conf.start_im) / conf.im_steps as f64;

        for i in 0..conf.im_steps {

            let im = conf.start_im + im_particle * i as f64;

            for j in 0..conf.re_steps {
                let re = conf.start_re + re_particle * j as f64;
                result.push(self.convergence(ComplexF64::new(re, im), conf.max_iters))
            }
        }

        return result;
    }
}