#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorFRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl ColorFRGB {
    pub fn new(red: f32, green: f32, blue: f32) -> Option<ColorFRGB> {
        if red > 1.0 || red < 0.0 || green > 1.0 || green < 0.0 || blue > 1.0 || blue < 0.0 {
            None
        } else {
            Some(ColorFRGB {
                r: red,
                g: green,
                b: blue,
            })
        }
    }
}

impl From<ColorFHSV> for ColorFRGB {
    fn from(hsv: ColorFHSV) -> ColorFRGB {
        let chrominance = hsv.v * hsv.s;
        let sextant = (hsv.h / 60.0).floor() as u8;
        let min = hsv.v - chrominance;
        let mid = (chrominance * (1.0 - (((hsv.h / 60.0) % 2.0) - 1.0).abs())) + min;
        let max = chrominance + min; //should equal value.
        match sextant {
            0 => ColorFRGB::new(max, mid, min)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            1 => ColorFRGB::new(mid, max, min)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            2 => ColorFRGB::new(min, max, mid)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            3 => ColorFRGB::new(min, mid, max)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            4 => ColorFRGB::new(mid, min, max)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            5 => ColorFRGB::new(max, min, mid)
                .unwrap_or_else(|| panic!("Error in HSV to RGB conversion. This is a bug.")),
            _ => panic!("Invalid sextant in HSV to RGB conversion. This is a bug."),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorFHSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl ColorFHSV {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Option<ColorFHSV> {
        if hue > 360.0
            || hue < 0.0
            || saturation > 1.0
            || saturation < 0.0
            || value > 1.0
            || value < 0.0
        {
            None
        } else {
            Some(ColorFHSV {
                h: hue,
                s: saturation,
                v: value,
            })
        }
    }
}

impl From<ColorFRGB> for ColorFHSV {
    fn from(c: ColorFRGB) -> ColorFHSV {
        let max = c.r.max(c.g.max(c.b));
        let min = c.r.min(c.g.min(c.b));
        let chrominance = max - min;
        let saturation = chrominance / max;
        let hue;
        if max == 0.0 {
            return ColorFHSV {
                h: 0.0,
                s: 0.0,
                v: 0.0,
            };
        }
        if chrominance == 0.0 {
            return ColorFHSV {
                h: 0.0,
                s: 0.0,
                v: max,
            };
        }
        if max == c.r {
            hue = 60.0 * (((c.g - c.b) / chrominance) % 6.0);
        } else if max == c.g {
            hue = 60.0 * (((c.b - c.r) / chrominance) + 2.0);
        } else {
            //max == c.b
            hue = 60.0 * (((c.r - c.g) / chrominance) + 4.0);
        }
        ColorFHSV::new(hue, saturation, max)
            .unwrap_or_else(|| panic!("Error in RGB to HSV conversion. This is a bug."))
    }
}
