extern crate lodepng;
use color::*;
use flame_2d;
use flame_2d::AffineTransform as Affine2d;
use flame_2d::Transform as Transform2d;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time;
use std::time::Instant;

#[test]
fn fRGB_to_fHSV_and_back() {
    let red = ColorFRGB::new(1.0, 0.0, 0.0).unwrap();
    let green = ColorFRGB::new(1.0, 0.0, 0.0).unwrap();
    let blue = ColorFHSV::new(240.0, 1.0, 1.0).unwrap();

    let redHSV = ColorFHSV::from(red);
    let greenHSV = ColorFHSV::from(green);
    let blueRGB = ColorFRGB::from(blue);

    assert_eq!(ColorFRGB::from(redHSV), red);
    assert_eq!(ColorFRGB::from(greenHSV), green);
    assert_eq!(ColorFHSV::from(blueRGB), blue);

    let yellow = ColorFRGB::new(1.0, 1.0, 0.0).unwrap();
    let orange = ColorFHSV::new(30.0, 1.0, 1.0).unwrap();
    let desat = ColorFHSV::new(70.0, 0.3, 0.7).unwrap();

    let yellowHSV = ColorFHSV::from(yellow);
    let orangeRGB = ColorFRGB::from(orange);
    let desatRGB = ColorFRGB::from(desat);

    assert_eq!(ColorFRGB::from(yellowHSV), yellow);
    assert_eq!(ColorFHSV::from(orangeRGB), orange);

    //Breaking it apart for some teeny tolerance.
    let desat_back = ColorFHSV::from(desatRGB);
    assert!((desat_back.h - desat.h).abs() < 0.0001);
    assert!((desat_back.s - desat.s).abs() < 0.0001);
    assert!((desat_back.v - desat.v).abs() < 0.0001);

    //TODO: More checks here.
}

#[test]
fn basic_flame_render_test() {
    let start = Instant::now();
    let mut flame = flame_2d::Flame::new(String::from("Testflame"));
    let xf01 = Transform2d::AffineOnly(Affine2d {
        xx: 0.95,
        xy: 0.1,
        yx: 0.1,
        yy: 0.975,
        cx: 0.05,
        cy: 0.1,
    });
    let xf02 = Transform2d::AffineOnly(Affine2d {
        xx: 0.1,
        xy: 0.8,
        yx: 0.7,
        yy: 0.5,
        cx: 0.0,
        cy: 0.01,
    });
    let xf03 = Transform2d::AffineOnly(Affine2d {
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
