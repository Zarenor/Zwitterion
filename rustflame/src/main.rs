#![feature(get_type_id)]
#![feature(assoc_unix_epoch)]
extern crate lodepng;
extern crate rustflame;

use rustflame::Config;

use rustflame::color::ColorFRGB;
use rustflame::flame_2d;
use rustflame::flame_2d::AffineTransform as Affine2d;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //let startupconfig;
    println!("rustflame starting up...");
    quickrun();
    /*
    match Config::new(&args) {
        Ok(cfg) => {
            startupconfig = cfg;
        }
        Err(e) => {
            println!("Error! {}", e);
            process::exit(16);
        }
    }*/
    //if let Err(e) = rustflame::run(startupconfig) {}
    println!("rustflame shutting down...")
}

fn quickrun() {
    let start = Instant::now();
    let mut flame = flame_2d::Flame::new(String::from("Testflame"));
    let xf01 = flame_2d::Transform::AffineOnly(Affine2d {
        xx: 0.95,
        xy: 0.1,
        yx: 0.1,
        yy: 0.975,
        cx: 0.05,
        cy: 0.1,
    });
    let xf02 = flame_2d::Transform::AffineOnly(Affine2d {
        xx: 0.1,
        xy: 0.8,
        yx: 0.7,
        yy: 0.5,
        cx: 0.0,
        cy: 0.01,
    });
    let xf03 = flame_2d::Transform::AffineOnly(Affine2d {
        xx: 0.9,
        xy: 0.05,
        yx: 0.05,
        yy: 0.9,
        cx: 0.0,
        cy: 0.0,
    });
    let c01 = ColorFRGB::new(1.0, 1.0, 1.0).unwrap();
    let c02 = ColorFRGB::new(1.0, 0.0, 0.0).unwrap();
    let c03 = ColorFRGB::new(0.0, 1.0, 1.0).unwrap();
    flame.add_transform(xf01, c01);
    flame.add_transform(xf02, c02);
    flame.add_transform(xf03, c03);
    let mut renderer = flame_2d::Renderer::new(1920, 1080, 2.2, 0.75, flame);
    let bm = renderer.render_unthreaded();
    println!("Copied the bitmap out");

    let pathstring = format!(
        "{} render.png",
        time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let path = Path::new(&pathstring);
    println!(
        "{} seconds elapsed. Got the path: {:?}, beginning encoding...",
        start.elapsed().as_secs(),
        path
    );
    let enc = lodepng::encode24(&bm.buffer, bm.width, bm.height).unwrap();
    println!(
        "{} seconds elapsed. Finished encoding. Saving file...",
        start.elapsed().as_secs()
    );
    let mut file = File::create(path).unwrap();
    file.write_all(&enc);
    println!("File saved. {} seconds elapsed.", start.elapsed().as_secs())
}
