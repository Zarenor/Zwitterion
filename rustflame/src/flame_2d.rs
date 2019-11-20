extern crate lodepng;
extern crate rand;
extern crate rgb;

use self::lodepng::Bitmap;
use self::lodepng::RGB;
use crate::color::ColorFRGB;
use rand::Rng;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Mul;
use std::time::Duration;
use std::time::Instant;

pub struct Flame {
    pub name: String,
    pub transforms: Vec<Transform>,
    pub colors: Vec<ColorFRGB>,
    //Plenty more to be added later
    //final transform(s)
    //color associations
    //conversion to rendering form? May do that. fully-immutable and sharable.
    //for now, shared factor here - it's all the same impl.
}

impl Flame {
    pub fn new(name: String) -> Flame {
        Flame {
            name,
            transforms: Vec::new(),
            colors: Vec::new(),
        }
    }
    pub fn add_transform(&mut self, t: Transform, c: ColorFRGB) {
        self.transforms.push(t);
        self.colors.push(c);
    }
}

pub enum Transform {
    AffineOnly(AffineTransform),              //Linear, basically
    Chaining(Box<Transform>, Box<Transform>), //For ridic shit... but we don't handle an affine at all here. kinda odd?
    Basic(AffineTransform, Box<dyn TransformFunction>),
    Sum(AffineTransform, Vec<(f64, Box<dyn TransformFunction>)>), //The thing we all know, love, and have come to expect elsewhere.
}

impl Transform {
    fn transform(&self, input: &Point) -> Point {
        match self {
            Transform::AffineOnly(affine) => affine.transform(input),
            Transform::Basic(affine, transform) => transform.transform(&affine.transform(input)),
            Transform::Chaining(transform1, transform2) => {
                transform2.transform(&transform1.transform(input))
            }
            Transform::Sum(affine, vec) => {
                let p0 = affine.transform(input);
                vec.into_iter()
                    .map(|(f, x)| x.transform(&p0) * f.clone())
                    .sum()
            }
        }
    }
}

pub struct AffineTransform {
    pub xx: f64,
    pub xy: f64,
    pub yx: f64,
    pub yy: f64,
    pub cx: f64,
    pub cy: f64,
}

impl AffineTransform {
    fn transform(&self, input: &Point) -> Point {
        Point {
            x: input.x * self.xx + input.y * self.yx + self.cx,
            y: input.x * self.xy + input.y * self.yy + self.cy,
        }
    }
}

pub trait TransformFunction {
    fn transform(&self, input: &Point) -> Point;
}
struct SinusoidalTransform;
impl TransformFunction for SinusoidalTransform {
    fn transform(&self, input: &Point) -> Point {
        Point {
            x: input.x.sin(),
            y: input.y.sin(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl<'a> Mul<&'a f64> for Point {
    type Output = Point;
    fn mul(self, rhs: &'a f64) -> Point {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Sum for Point {
    fn sum<I: Iterator<Item = Point>>(iter: I) -> Point {
        iter.fold(Point { x: 0.0, y: 0.0 }, |a, b| a + b)
    }
}

#[derive(Copy, Clone, Default)]
struct RenderBin {
    r: f64,
    g: f64,
    b: f64,
    h: u64,
}

impl RenderBin {
    fn register_hit(&mut self, c: ColorFRGB) {
        self.h += 1;
        //Rather than do multiplications or divisions, we sum the colors, and renormalize later.
        self.r += c.r as f64;
        self.g += c.g as f64;
        self.b += c.b as f64;
    }
    fn normalize(&mut self) {
        let fh = self.h as f64;
        self.r = self.r / fh;
        self.g = self.g / fh;
        self.b = self.b / fh;
    }
    fn normalize_by(&mut self, i: u64) {
        let fi = i as f64;
        self.r = self.r / fi;
        self.g = self.g / fi;
        self.b = self.b / fi;
    }
}

pub struct Renderer {
    image_width: u32,
    image_height: u32,
    gamma: f64,
    vibrancy: f64,
    flame: Flame,
}

//TODO: Create a RendererBuilder
impl Renderer {
    pub fn new(
        image_width: u32,
        image_height: u32,
        gamma: f64,
        vibrancy: f64,
        flame: Flame,
    ) -> Renderer {
        Renderer {
            image_width,
            image_height,
            gamma,
            vibrancy,
            flame,
        }
    }
    pub fn render_unthreaded(&mut self) -> Bitmap<RGB<u8>> {
        let render_start = Instant::now();
        //TODO: consider using an intermediate struct to return to allow for post-processing.
        let pixels = (self.image_width * self.image_height) as usize;
        let mut render_array = Vec::<RenderBin>::with_capacity(pixels);
        for _p in 0..pixels {
            //Hopefully this does this right,and doesn't do anything ludicrous.
            render_array.push(RenderBin::default())
        }
        println!(
            "render_array initialized with {} elements, expected {}",
            render_array.len(),
            pixels
        );
        //TODO: Consider render areas other than the biunit square.
        let dw = 2.0 / self.image_width as f64;
        let dh = 2.0 / self.image_height as f64;
        let mut points = Vec::<Point>::new();
        for i in 0..10 {
            for j in 0..10 {
                points.push(Point {
                    x: i as f64 * 0.1,
                    y: j as f64 * 0.1,
                });
                if i != 0 || j != 0 {
                    points.push(Point {
                        x: i as f64 * -0.1,
                        y: j as f64 * -0.1,
                    });
                }
            }
        }
        println!("{} starting points generated", points.len());
        //TODO: consider a bigint for iterations.
        let mut iterations = 0usize;
        let mut keep_looping = true;
        let iterate_start = Instant::now();
        let iterate_duration = Duration::from_secs(60);
        let num_xforms = self.flame.transforms.len();
        let mut selected_xform: usize;
        let mut rng = rand::thread_rng();
        println!(
            "[{:?}] Starting iteration with {} transforms, {} points",
            render_start.elapsed(),
            num_xforms,
            points.len()
        );
        //for now, hardcoding termination based on time.
        //TODO: Termination options, stored as part of the struct at creation. Cancellation.

        while keep_looping {
            let sel = iterations % points.len();
            let p0 = points[sel];
            selected_xform = rng.gen::<u32>() as usize % num_xforms;
            let p1 = self.flame.transforms[selected_xform].transform(&p0);
            if p1.x.abs() > 1.0 || p1.y.abs() > 1.0 {
                //ignore this transformation if it falls outside -
                //for now, this means that point is unaffected and will receive another transform later.
                //We may change that ro replacement with a new, random point that isn't plotted.
            } else {
                //Plot the point
                let px = p1.x + 1.0;
                let py = p1.y + 1.0;
                let bx = px / dw;
                let by = py / dh;
                let idx = bx.trunc() as usize + (self.image_width * by.trunc() as u32) as usize;
                if idx < render_array.len() {
                    render_array[idx].register_hit(self.flame.colors[selected_xform]);
                } else {
                    println!(
                        "[{:?}] Attempted to plot point out of range! idx:{} len:{} bx:{} by{}",
                        render_start.elapsed(),
                        idx,
                        render_array.len(),
                        bx,
                        by
                    );
                }
                points[sel] = p1;
            }
            iterations += 1;
            //Stop iterating when the duration has elapsed.
            keep_looping = iterate_start.elapsed() < iterate_duration
        }
        println!(
            "[{:?}] Stopped iterating, {} iterations complete",
            render_start.elapsed(),
            iterations
        );
        let mut max_hits = 0u64;
        for bin in &render_array {
            if bin.h > max_hits {
                max_hits = bin.h;
            }
        }
        let mut pixel_buffer = Vec::<RGB<u8>>::with_capacity(pixels);
        for mut bin in render_array {
            //The ordering here is enforced,right? goodness, I hope so.
            bin.normalize_by(max_hits);
            let rf = ((bin.h as f64).powf(1.0 / self.gamma) * self.vibrancy * bin.r)
                + (bin.r.powf(1.0 / self.gamma) * (1.0 - self.vibrancy));
            let gf = ((bin.h as f64).powf(1.0 / self.gamma) * self.vibrancy * bin.g)
                + (bin.g.powf(1.0 / self.gamma) * (1.0 - self.vibrancy));
            let bf = ((bin.h as f64).powf(1.0 / self.gamma) * self.vibrancy * bin.b)
                + (bin.b.powf(1.0 / self.gamma) * (1.0 - self.vibrancy));

            let r = if rf <= 0.0 {
                0u8
            } else if rf >= 1.0 {
                255u8
            } else {
                (rf * 255.0) as u8
            };
            let g = if gf <= 0.0 {
                0u8
            } else if gf >= 1.0 {
                255u8
            } else {
                (gf * 255.0) as u8
            };
            let b = if bf <= 0.0 {
                0u8
            } else if bf >= 1.0 {
                255u8
            } else {
                (bf * 255.0) as u8
            };

            let pixel = RGB { r, g, b };
            pixel_buffer.push(pixel);
        }

        let ret = Bitmap {
            buffer: pixel_buffer,
            width: self.image_width as usize,
            height: self.image_height as usize,
        };
        println!("[{:?}] Finished render", render_start.elapsed());
        ret
    }
}
