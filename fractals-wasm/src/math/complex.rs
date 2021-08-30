#[derive(Copy, Clone)]
pub struct ComplexF64 {
    re: f64,
    im: f64,
}

impl ComplexF64 {

    pub fn new(re: f64, im: f64) -> ComplexF64 {
        ComplexF64 {
            re, im
        }
    }

    #[inline(always)]
    pub fn re(&self) -> f64 {
        self.re
    }

    #[inline(always)]
    pub fn im(&self) -> f64 {
        self.im
    }
}