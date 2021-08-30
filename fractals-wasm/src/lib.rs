mod math;
mod color;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate console_error_panic_hook;
extern crate palette;

use wasm_bindgen::prelude::*;
use math::{Generator, FractalConfig};
use color::{ColorTransformation, HsvBasedColorTransformation, SmoothColorTransformation, HsvTransMode};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Resolution {
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
impl Resolution {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct ComplexPlaneRange {
    pub start_im: f64,
    pub end_im: f64,

    pub start_re: f64,
    pub end_re: f64
}

#[wasm_bindgen]
impl ComplexPlaneRange {

    pub fn new(start_re: f64, end_re: f64, start_im: f64, end_im: f64) -> Self {
        ComplexPlaneRange {
            start_im, end_im, start_re, end_re
        }
    }
}

#[wasm_bindgen]
pub struct GeneratorConfig {
    name: String,
    args: [f64; 2]
}

#[wasm_bindgen]
impl GeneratorConfig {
    pub fn mandelbrot() -> Self {
        Self {
            name: String::from("Mandelbrot"),
            args: [0.0, 0.0]
        }
    }

    pub fn julia_set(re: f64, im: f64) -> Self {
        Self {
            name: String::from("JuliaSet"),
            args: [re, im]
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        return self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn arg(&self) -> Vec<f64> {
        return vec![self.args[0], self.args[1]]
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    #[wasm_bindgen(setter)]
    pub fn set_arg(&mut self, args: Vec<f64>) {
        self.args = [
            *args.get(0).expect(""),
            *args.get(1).expect("")
        ]
    }
}

impl GeneratorConfig {
    pub fn as_generator(&self) -> Generator {
        return match self.name.as_str() {
            "Mandelbrot" => Generator::MANDELBROT,
            "JuliaSet" => Generator::JULIA(
                self.args[0],
                self.args[1],
            ),
            _ => panic!("Unrecognized generator!")
        }
    }
}

#[wasm_bindgen]
pub struct FramePartConfig {
    pub res: Resolution,
    pub plane: ComplexPlaneRange,
    pub scaling: usize,
    pub part_num: usize,
    pub part_count: usize,
    pub max_iters: usize,
}

#[wasm_bindgen]
impl FramePartConfig {

    pub fn new (
        res: Resolution,
        plane: ComplexPlaneRange,
        scaling: usize,
        part_num: usize,
        part_count: usize,
        max_iters: usize
    ) -> Self {
        return Self {
            res,
            plane,
            scaling,
            part_num,
            part_count,
            max_iters
        }
    }

    pub fn is_last_leftover(&self) -> bool {
        self.res.height % self.part_count != 0 && self.part_num == self.part_count
    }

    pub fn height_split(&self) -> usize {
        self.descaled_height() / self.part_count
    }

    pub fn height_leftover(&self) -> usize {
        self.descaled_height() % self.part_count
    }

    pub fn descaled_width(&self) -> usize {
        if self.res.width % self.scaling != 0 {
            panic!("Width must be divisible by scaling!")
        }

        return self.res.width / self.scaling;
    }

    pub fn descaled_height(&self) -> usize {
        if self.res.height % self.scaling != 0 {
            panic!("Height must be divisible by scaling!")
        }

        return self.res.height / self.scaling;
    }

    pub fn frame_height(&self) -> usize {
        return if self.is_last_leftover() {
            self.height_leftover()
        } else {
            self.height_split()
        };
    }

    pub fn frame_complex_plane(&self) -> ComplexPlaneRange {
        let frame_height = self.frame_height();
        let current_factor = frame_height as f64 / self.descaled_height() as f64;
        let im_length = (self.plane.end_im - self.plane.start_im) * current_factor;

        let factor = (self.part_num * self.height_split()) as f64 / self.descaled_height() as f64;
        let im_start = (self.plane.end_im - self.plane.start_im) * factor;

        return ComplexPlaneRange {
            start_im: self.plane.start_im + im_start,
            end_im: self.plane.start_im + im_start + im_length,
            start_re: self.plane.start_re,
            end_re: self.plane.end_re
        }
    }
}

#[wasm_bindgen]
pub enum ColorTransMode {
    Hue, Saturation, Value
}

impl ColorTransMode {
    pub fn adapt(&self) -> HsvTransMode {
        return match self {
            ColorTransMode::Hue => HsvTransMode::Hue,
            ColorTransMode::Saturation => HsvTransMode::Saturation,
            ColorTransMode::Value => HsvTransMode::Value
        }
    }
}

#[wasm_bindgen]
pub struct ColorConfig {
    h_base: f32,
    s_base: f32,
    v_base: f32,
    mode: ColorTransMode,
    smooth: bool
}

#[wasm_bindgen]
impl ColorConfig {
    pub fn new(
        h_base: f32,
        s_base: f32,
        v_base: f32,
        mode: ColorTransMode,
        smooth: bool
    ) -> Self {
        Self {
            h_base,
            s_base,
            v_base,
            mode,
            smooth
        }
    }
}

#[wasm_bindgen]
pub fn generate_frame_part(
    config: &FramePartConfig,
    generator: &GeneratorConfig,
    color_config: &ColorConfig
) -> Vec<u8> {
    if config.res.height % config.scaling != 0 {
        panic!("Height must be divisible by scaling!")
    }

    let gen = generator
        .as_generator()
        .create();

    let frame_plane = config.frame_complex_plane();

    let f_conf = FractalConfig {
        start_re: frame_plane.start_re,
        end_re: frame_plane.end_re,
        re_steps: config.descaled_width(),

        start_im: frame_plane.start_im,
        end_im: frame_plane.end_im,
        im_steps: config.frame_height(),

        max_iters: config.max_iters,
    };

    let result = gen.generate(&f_conf);

    let base_trans = HsvBasedColorTransformation {
        h_base: color_config.h_base,
        s_base: color_config.s_base,
        v_base: color_config.v_base,
        mode: color_config.mode.adapt()
    };

    let trans: Box<dyn ColorTransformation> = if color_config.smooth {
        Box::new(SmoothColorTransformation { base: base_trans })
    } else {
        Box::new(base_trans)
    };

    let colored = trans.transform(
        result,
        f_conf.max_iters,
        config.descaled_width(),
        config.frame_height(),
        config.scaling
    );

    return colored;
}