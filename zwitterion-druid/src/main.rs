use druid::widget::{SizedBox, Split};
use druid::{AppLauncher, Data, LocalizedString, Widget, WindowDesc};

fn main() {
    let title = LocalizedString::new("title").with_placeholder("Zwitterion Druid Test".to_string());

    let window = WindowDesc::new(transform_edit_window).title(title);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(create_data())
        .expect("Launch Failed!");
}

fn create_data() -> Flame {
    Flame {
        name: "Flame 1".to_string(),
        transforms: vec![Transform::Linear(AffineTransform {
            xx: 0.5,
            xy: 0.0,
            yx: 0.0,
            yy: 0.5,
            cx: 0.0,
            cy: 0.0,
        })],
    }
}

fn transform_edit_window() -> impl Widget<Flame> {
    let left_panel = SizedBox::empty().expand();
    let right_panel = SizedBox::empty().expand();
    let root = Split::horizontal(left_panel, right_panel).split_point(0.7);
    root
}
#[derive(Clone)]
struct Flame {
    name: String,
    transforms: Vec<Transform>,
    //TODO: Worry about colors, other options.
}

impl Data for Flame {
    fn same(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.transforms.len() != other.transforms.len() {
            return false;
        }
        let zip = self.transforms.iter().zip(other.transforms.iter());
        for (transform1, transform2) in zip {
            if transform1 != transform2 {
                return false;
            }
        }
        true
    }
}

#[derive(Data, Clone, PartialEq)]
enum Transform {
    Linear(AffineTransform),
    //Many, many more to come, plus plugin extensibility.
}

#[derive(Data, Clone, PartialEq)]
struct AffineTransform {
    xx: f64,
    xy: f64,
    yx: f64,
    yy: f64,
    cx: f64,
    cy: f64,
}
