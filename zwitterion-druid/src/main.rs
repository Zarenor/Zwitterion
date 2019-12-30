use druid::kurbo::Size;
use druid::lens::{Field, Id, Map};
use druid::widget::{Flex, Label, LabelText, Parse, SizedBox, Split, TextBox, WidgetExt};
use druid::*;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;

fn main() {
    let title = LocalizedString::new("title").with_placeholder("Zwitterion Druid Test".to_string());

    let window = WindowDesc::new(TransformEditWindow::new).title(title);
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

struct TransformEditWindow {
    split: Split<Flame>,
}

impl TransformEditWindow {
    fn new() -> Self {
        let left_panel = transform_canvas();
        let right_panel = transform_edit_panel();
        let split = Split::horizontal(left_panel, right_panel)
            .split_point(0.7)
            .draggable(true);
        Self { split }
    }
}

impl Widget<Flame> for TransformEditWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Flame, env: &Env) {
        let old_data = data.clone();
        match &old_data.selected_transform{
            Some(xform) =>{
                let old_xform = Transform::clone(&xform);
                let xform_string = format!("{:?}", old_xform);
                
            }
            None =>{
                println!("Should be unreachable!")
            }
        }
        self.split.event(ctx, event, data, env);
        if !old_data.same(data) {
            ctx.invalidate();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&Flame>, data: &Flame, env: &Env) {
        self.split.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Flame,
        env: &Env,
    ) -> Size {
        self.split.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &Flame, env: &Env) {
        self.split.paint(paint_ctx, data, env);
    }
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
            *Arc::make_mut(arc) = lensed;
            *data = Some(arc.clone());
        }
    }
}

fn get_focus_affine() -> impl Lens<Option<Arc<Transform>>, AffineTransform> + Clone {
    let focus_affine = Map::new(unarc_get, unarc_put);
    let focus_affine = focus_affine.then(Map::new(get_affine, put_affine));
    focus_affine
}

fn get_affine(t: &Transform) -> AffineTransform {
    t.affine_transform().clone()
}

fn put_affine(t: &mut Transform, f: AffineTransform) {
    t.set_affine_transform(f);
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
        let flex_xx = LabeledTextBox::new("xx:");
        let flex_xy = LabeledTextBox::new("xy:");
        let flex_yx = LabeledTextBox::new("yx:");
        let flex_yy = LabeledTextBox::new("yy:");
        let flex_cx = LabeledTextBox::new("cx:");
        let flex_cy = LabeledTextBox::new("cy:");
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
        let old_data = data.clone();
        self.flex_col.event(ctx, event, data, env);
        if !old_data.same(data) {
            ctx.invalidate();
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: Option<&AffineTransform>,
        data: &AffineTransform,
        env: &Env,
    ) {
        match old_data {
            None => ctx.invalidate(),
            Some(old) => {
                if !old.same(&data) {
                    ctx.invalidate();
                }
            }
        }
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
        if !self.selected_transform.same(&other.selected_transform)
        {return false;}
        let zip = self.transforms.iter().zip(other.transforms.iter());
        for (transform1, transform2) in zip {
            if !transform1.same(transform2) {
                return false;
            }
        }
        true
    }
}

#[derive(Data, Clone, PartialEq,Debug)]
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
        match self {
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

struct LabeledTextBox<T: 'static + FromStr + Display + Data> {
    flex_row: Flex<T>,
}

impl<T: 'static + FromStr + Display + Data> LabeledTextBox<T> {
    fn new(label_text: impl Into<LabelText<T>>) -> Self {
        let label = Label::new(label_text);
        let edit = LensWrap::new(Parse::new(TextBox::new().padding(3.0)), unoption_map());
        let flex_row = Flex::row().with_child(label, 0.0).with_child(edit, 1.0);
        Self { flex_row }
    }
}

impl<T: 'static + FromStr + Display + Data> Widget<T> for LabeledTextBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.flex_row.event(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        match old_data {
            None => ctx.invalidate(),
            Some(old) => {
                if !old.same(&data) {
                    ctx.invalidate();
                }
            }
        }
        self.flex_row.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.flex_row.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.flex_row.paint(paint_ctx, data, env);
    }
}
