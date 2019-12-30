use druid::kurbo::Size;
use druid::lens::{Field, Id, Map};
use druid::widget::{Flex, Label, Parse, SizedBox, Split, TextBox, WidgetExt};
use druid::*;
use std::ops::Deref;
use std::sync::Arc;

fn main() {
    let title = LocalizedString::new("title").with_placeholder("Zwitterion Druid Test".to_string());

    let window = WindowDesc::new(transform_edit_window).title(title);
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(create_data())
        .expect("Launch Failed!");
}

fn create_data() -> Flame {
    let xform_0 = Arc::new(Transform::default());
    Flame {
        name: "Flame 1".to_string(),
        transforms: vec![xform_0.clone()],
        selected_transform: Some(xform_0),
    }
}

fn transform_edit_window() -> impl Widget<Flame> {
    let left_panel = transform_canvas();
    let right_panel = transform_edit_panel();
    let root = Split::horizontal(left_panel, right_panel)
        .split_point(0.7)
        .draggable(true);
    root
}

fn transform_edit_panel() -> impl Widget<Flame> {
    Split::vertical(
        transform_list_panel(),
        LensWrap::new(
            LensWrap::new(
                Split::vertical(TransformCoordsPanel::new(), transform_variations_panel())
                    .draggable(true),
                get_focus_affine(),
            ),
            lens!(Flame, selected_transform),
        ),
    )
    .split_point(0.4)
    .draggable(true)
}

fn transform_list_panel() -> impl Widget<Flame> {
    SizedBox::empty().expand()
}

fn unarc_get<T: Data + Default>(data: &Option<Arc<T>>) -> T {
    match data {
        Some(arc) => T::clone(&arc),
        None => Default::default(),
    }
}

fn unarc_put<T: Data + Default>(data: &mut Option<Arc<T>>, lensed: T) {
    //If there's no data here, there's nothing selected to edit.
    if let Some(arc) = data {
        if !T::same(&arc, &lensed) {
            let mut data = Arc::make_mut(arc);
            *data = lensed;
        }
    }
}

fn get_focus_affine() -> impl Lens<Option<Arc<Transform>>, AffineTransform> + Clone {
    let focus_affine = Map::new(unarc_get, unarc_put);
    let focus_affine = focus_affine.then(Map::new(
        |t: &Transform| t.affine_transform().clone(),
        |t: &mut Transform, f: AffineTransform| t.set_affine_transform(f),
    ));
    focus_affine
}
fn unoption_map<T: Data>() -> impl Lens<T, Option<T>> {
    Map::new(
        |t: &T| Some(t.clone()),
        |t: &mut T, u: Option<T>| {
            if let Some(u) = u {
                *t = u;
            }
        },
    )
}
struct TransformCoordsPanel {
    flex_col: Flex<AffineTransform>,
}
impl TransformCoordsPanel {
    fn new() -> Self {
        let label_xx = Label::new("xx:");
        let label_xy = Label::new("xy:");
        let label_yx = Label::new("yx:");
        let label_yy = Label::new("yy:");
        let label_cx = Label::new("cx:");
        let label_cy = Label::new("cy:");
        let editbox = || LensWrap::new(Parse::new(TextBox::new().padding(3.0)), unoption_map());
        let flex_xx = Flex::row()
            .with_child(label_xx, 0.0)
            .with_child(editbox(), 1.0);
        let flex_xy = Flex::row()
            .with_child(label_xy, 0.0)
            .with_child(editbox(), 1.0);
        let flex_yx = Flex::row()
            .with_child(label_yx, 0.0)
            .with_child(editbox(), 1.0);
        let flex_yy = Flex::row()
            .with_child(label_yy, 0.0)
            .with_child(editbox(), 1.0);
        let flex_cx = Flex::row()
            .with_child(label_cx, 0.0)
            .with_child(editbox(), 1.0);
        let flex_cy = Flex::row()
            .with_child(label_cy, 0.0)
            .with_child(editbox(), 1.0);
        let row_x = Flex::row()
            .with_child(LensWrap::new(flex_xx, lens!(AffineTransform, xx)), 0.5)
            .with_child(LensWrap::new(flex_xy, lens!(AffineTransform, xy)), 0.5);
        let row_y = Flex::row()
            .with_child(LensWrap::new(flex_yx, lens!(AffineTransform, yx)), 0.5)
            .with_child(LensWrap::new(flex_yy, lens!(AffineTransform, yy)), 0.5);
        let row_c = Flex::row()
            .with_child(LensWrap::new(flex_cx, lens!(AffineTransform, cx)), 0.5)
            .with_child(LensWrap::new(flex_cy, lens!(AffineTransform, cy)), 0.5);
        let dynlabel = Label::new(|data: &AffineTransform, _env: &_| format!("{:?}", data));
        let flex_col = Flex::column()
            .with_child(row_x, 1.0)
            .with_child(row_y, 1.0)
            .with_child(row_c, 1.0)
            .with_child(dynlabel, 0.7);
        Self { flex_col }
    }
}
impl Widget<AffineTransform> for TransformCoordsPanel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AffineTransform, env: &Env) {
        self.flex_col.event(ctx, event, data, env)
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: Option<&AffineTransform>,
        data: &AffineTransform,
        env: &Env,
    ) {
        //unimplemented!()
        self.flex_col.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AffineTransform,
        env: &Env,
    ) -> Size {
        self.flex_col.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AffineTransform, env: &Env) {
        self.flex_col.paint(paint_ctx, data, env);
    }
}
fn transform_variations_panel() -> impl Widget<AffineTransform> {
    SizedBox::empty().expand()
}

fn transform_canvas() -> impl Widget<Flame> {
    SizedBox::empty().expand()
}

//TODO: Rename to reflect that this is the UI data model, not the flame struct itself
#[derive(Clone)]
struct Flame {
    name: String,
    transforms: Vec<Arc<Transform>>,
    selected_transform: Option<Arc<Transform>>,
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
            if !transform1.same(transform2) {
                return false;
            }
        }
        true
    }
}

#[derive(Data, Clone, PartialEq)]
enum Transform {
    Linear(AffineTransform),
    //Many, many more to come, plus (hopefully) plugin extensibility. And some way to introspect the names, or attach them as metadata.
}
impl Transform {
    fn affine_transform(&self) -> AffineTransform {
        match *self {
            Transform::Linear(xform) => xform,
        }
    }
    fn set_affine_transform(&mut self, xform: AffineTransform) {
        match *self {
            Transform::Linear(mut xf) => xf = xform,
        }
    }
}
impl Default for Transform {
    fn default() -> Self {
        Transform::Linear(Default::default())
    }
}
#[derive(Data, Clone, Copy, PartialEq, Lens, Debug)]
struct AffineTransform {
    pub xx: f64,
    pub xy: f64,
    pub yx: f64,
    pub yy: f64,
    pub cx: f64,
    pub cy: f64,
}
impl Default for AffineTransform {
    fn default() -> Self {
        Self {
            xx: 0.5,
            xy: 0.0,
            yx: 0.0,
            yy: 0.5,
            cx: 0.0,
            cy: 0.0,
        }
    }
}
#[derive(Clone, Copy)]
struct UnitLens;
impl UnitLens {
    fn new() -> Self {
        Self
    }
}
impl<T: ?Sized> Lens<T, ()> for UnitLens {
    fn with<V, F: FnOnce(&()) -> V>(&self, data: &T, f: F) -> V {
        f(&())
    }

    fn with_mut<V, F: FnOnce(&mut ()) -> V>(&self, data: &mut T, f: F) -> V {
        f(&mut ())
    }
}
