use druid::kurbo::Size;
use druid::lens::{Field, Id, Map};
use druid::widget::{
    Container, Flex, Label, LabelText, Padding, Parse, SizedBox, Split, TextBox, WidgetExt,
};
use druid::*;
use rustflame::color::ColorFHSV;
use std::f64::INFINITY;
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
        let split = Split::columns(left_panel, right_panel)
            .split_point(0.7)
            .draggable(true);
        Self { split }
    }
}

impl Widget<Flame> for TransformEditWindow {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Flame, env: &Env) {
        let old_data = data.clone();
        match &old_data.selected_transform {
            Some(xform) => {
                let old_xform = Transform::clone(&xform);
                let xform_string = format!("{:?}", old_xform);
            }
            None => println!("Should be unreachable!"),
        }
        self.split.event(ctx, event, data, env);
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Flame, env: &Env) {
        self.split.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Flame, data: &Flame, env: &Env) {
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
    Split::rows(
        transform_list_panel(),
        LensWrap::new(
            LensWrap::new(
                Split::rows(
                    TransformCoordsPanel::new().lens(affine_lens()),
                    transform_variations_panel(),
                )
                .draggable(true),
                unarc_lens(),
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

fn affine_lens() -> impl Lens<Transform, AffineTransform> + Clone {
    Map::new(get_affine, put_affine)
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
            .with_flex_child(flex_xx.lens(AffineTransform::xx), 0.5)
            .with_flex_child(flex_xy.lens(AffineTransform::xy), 0.5);
        let row_y = Flex::row()
            .with_flex_child(flex_yx.lens(AffineTransform::yx), 0.5)
            .with_flex_child(flex_yy.lens(AffineTransform::yy), 0.5);
        let row_c = Flex::row()
            .with_flex_child(flex_cx.lens(AffineTransform::cx), 0.5)
            .with_flex_child(flex_cy.lens(AffineTransform::cy), 0.5);
        let flex_col = Flex::column()
            .with_flex_child(row_x, 1.0)
            .with_flex_child(row_y, 1.0)
            .with_flex_child(row_c, 1.0);
        Self { flex_col }
    }
}
impl Widget<AffineTransform> for TransformCoordsPanel {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AffineTransform, env: &Env) {
        let old_data = data.clone();
        self.flex_col.event(ctx, event, data, env);
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AffineTransform,
        env: &Env,
    ) {
        self.flex_col.lifecycle(ctx, event, data, env);
    }
    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AffineTransform,
        data: &AffineTransform,
        env: &Env,
    ) {
        if !old_data.same(&data) {
            ctx.request_paint();
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

fn transform_variations_panel() -> impl Widget<Transform> {
    SizedBox::empty().expand()
}

fn transform_canvas() -> impl Widget<Flame> {
    let mut canvas = Canvas::new(Rect::new(-50.0, -50.0, 50.0, 50.0));
    let mut label = Label::new("text");
    label.set_text_size(2.0);
    let moving_label = CanvasWrap::new(label, |f: &Flame| match &f.selected_transform {
        Some(t) => {
            let a = t.affine_transform();
            println!("{} {} ", a.cx, a.cy);
            Point::new(a.cx, a.cy)
        }
        None => {
            println!("None in label closure");
            Point::ZERO
        }
    });
    canvas.add_child(moving_label);
    Padding::new(
        5.0,
        Container::new(canvas)
            .border(Color::grey8(64), 1.0)
            .background(Color::BLACK),
    )
}

//TODO: Rename to reflect that this is the UI data model, not the flame struct itself
#[derive(Clone, Lens)]
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
        if !self.selected_transform.same(&other.selected_transform) {
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

#[derive(Data, Clone, PartialEq, Debug)]
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
            Transform::Linear(ref mut xf) => *xf = xform,
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

fn unarc_lens<T: Data + Default>() -> impl Lens<Option<Arc<T>>, T> + Clone {
    Map::new(unarc_get, unarc_put)
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

struct Canvas<T: Data> {
    visible_area: Rect,
    children: Vec<(Rect, Box<dyn CanvasLayout<T>>)>,
}
//TODO: Figure out dynamic visible area? Data-bound? Or only with pan/zoom?
// Constrain max/min zoom? Max/min extents?
impl<T: Data> Canvas<T> {
    fn new(visible_area: Rect) -> Self {
        Self {
            visible_area,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: impl CanvasLayout<T> + 'static) {
        self.children.push((Rect::ZERO, Box::new(child)));
    }
}

impl<T: Data> Widget<T> for Canvas<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        //TODO: Panning & Zooming
        for (_, child) in self.children.iter_mut() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for (_, child) in &mut self.children {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        for (_, child) in self.children.iter_mut() {
            child.update(ctx, old_data, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        //Child layouts aren't dependent on the layout we're given here.
        for (layout, child) in self.children.iter_mut() {
            let (origin, size) = child.canvas_layout(ctx, &self.visible_area, data, env);
            //It may be we *don't* actually need to store this, because we ignore it anyway.
            //It's important the child has layout called, though. We just might not need a return.
            *layout = Rect::from_origin_size(origin, size);
        }
        //The canvas will always take up the maximum amount of space.
        //We'll have to complain? if we're unbounded. Maybe.

        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        if let Err(e) = paint_ctx.save() {
            //error!("saving render context failed: {:?}", e);
            return;
        }
        let old_region = paint_ctx.region().to_rect();
        let desired_region = self.visible_area;
        let x_scale = old_region.width() / desired_region.width();
        let y_scale = old_region.height() / desired_region.height();
        let scale_value = x_scale.min(y_scale);
        let scale = Affine::scale(scale_value);
        //TODO: Center properly.
        let x_trans = (old_region.x0.min(old_region.x1) - desired_region.x0.min(desired_region.x1))
            * scale_value;
        let y_trans = (old_region.y0.min(old_region.y1) - desired_region.y0.min(desired_region.y1))
            * scale_value;
        let translate = Affine::translate((x_trans, y_trans));
        let transform = translate * scale;
        paint_ctx.transform(transform);
        println!("Desired rect: {:?}", desired_region);
        println!("Scales we got: {} {}", x_scale, y_scale);
        println!("scale:{:?}",scale);
        println!("translate:{:?}", translate);
        println!("Transform we got: {:?}", transform);
        let new_width = old_region.width() / x_scale.min(y_scale);
        let new_height = old_region.height() / x_scale.min(y_scale);
        let clip_rect = 
        println!("given region:{} expected size: {}, {}", old_region, new_width, new_height);
        paint_ctx.clip(clip_rect);
        for (_, child) in self.children.iter_mut() {
            //Maybe we don't need the layout at all?! The CanvasWrap should handle it.
            //The issue, I think, is going to be figuring out hot/cold, in things which aren't podded.
            child.paint(paint_ctx, data, env);
        }
        if let Err(e) = paint_ctx.restore() {
            //error!("restoring render context failed: {:?}", e);
        }
    }
}

trait CanvasLayout<T: Data>: Widget<T> {
    //TODO: Determine if this is the necessary and sufficient amount of information
    //NOTE: A canvas will only call this method for layout - layout won't be called by the canvas directly.
    fn canvas_layout(
        &mut self,
        ctx: &mut LayoutCtx,
        visible_area: &Rect,
        data: &T,
        env: &Env,
    ) -> (Point, Size);
}

struct CanvasWrap<W: Widget<T>, T: Data, F: Fn(&T) -> Point> {
    inner: WidgetPod<T, W>,
    closure: F,
}

impl<W: Widget<T>, T: Data, F: Fn(&T) -> Point> CanvasWrap<W, T, F> {
    fn new(widget: W, closure: F) -> Self {
        Self {
            inner: WidgetPod::new(widget),
            closure,
        }
    }
}

impl<W: Widget<T>, T: Data, F: Fn(&T) -> Point> CanvasLayout<T> for CanvasWrap<W, T, F> {
    fn canvas_layout(
        &mut self,
        ctx: &mut LayoutCtx,
        visible_area: &Rect,
        data: &T,
        env: &Env,
    ) -> (Point, Size) {
        let desired_origin = (self.closure)(data);
        let desired_size = self.inner.layout(
            ctx,
            &BoxConstraints::new(Size::ZERO, Size::new(INFINITY, INFINITY)),
            data,
            env,
        );
        println!("{} {}", desired_origin, desired_size);
        self.inner.set_layout_rect(
            ctx,
            data,
            env,
            Rect::from_origin_size(desired_origin, desired_size),
        );
        (desired_origin, desired_size)
    }
}

impl<W: Widget<T>, T: Data, F: Fn(&T) -> Point> Widget<T> for CanvasWrap<W, T, F> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
                self.inner.update(ctx, data, env);
        if(self.closure)(data) != (self.closure)(old_data)
        {
            ctx.request_layout();
            //println!("Repaint requested");
        }
    }

    //NOTE: This is not called when we're being layouted on a canvas, so we act transparently.
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(paint_ctx, data, env);
    }
}

struct Line {
    color: Color,
}

impl Line {
    fn get_size(data: &(Point, Point)) -> Size {
        let (p0, p1) = data;
        let xmin = p0.x.min(p1.x);
        let ymin = p0.y.min(p1.y);
        let width = p0.x.max(p1.x) - xmin;
        let height = p0.y.max(p1.y) - ymin;
        (width, height).into()
    }
}

impl Widget<(Point, Point)> for Line {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut (Point, Point), env: &Env) {
        //This is a draw-only widget, for now.
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &(Point, Point),
        env: &Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &(Point, Point),
        _data: &(Point, Point),
        env: &Env,
    ) {
        //Nothing necessary here - we draw whatever data we have
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &(Point, Point),
        env: &Env,
    ) -> Size {
        bc.constrain(Line::get_size(data))
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &(Point, Point), env: &Env) {
        //TODO: Fit the size requested.
        let line = kurbo::Line::new(data.0, data.1);
        let thickness = if paint_ctx.is_hot() { 2.0 } else { 1.0 };
        paint_ctx.stroke(line, &self.color, thickness)
    }
}

struct LabeledTextBox<T: 'static + FromStr + Display + Data> {
    flex_row: Flex<T>,
}

impl<T: 'static + FromStr + Display + Data> LabeledTextBox<T> {
    fn new(label_text: impl Into<LabelText<T>>) -> Self {
        let label = Label::new(label_text);
        let edit = LensWrap::new(Parse::new(TextBox::new().padding(3.0)), unoption_map());
        let flex_row = Flex::row()
            .with_flex_child(label, 0.0)
            .with_flex_child(edit, 1.0);
        Self { flex_row }
    }
}

impl<T: 'static + FromStr + Display + Data> Widget<T> for LabeledTextBox<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.flex_row.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.flex_row.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(&data) {
            ctx.request_paint()
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

mod lens {
    use druid::{Data, Lens};
    use std::sync::Arc;

    #[derive(Clone, Copy)]
    pub struct Unit;

    impl<T: ?Sized> Lens<T, ()> for Unit {
        fn with<V, F: FnOnce(&()) -> V>(&self, _data: &T, f: F) -> V {
            f(&())
        }

        fn with_mut<V, F: FnOnce(&mut ()) -> V>(&self, _data: &mut T, f: F) -> V {
            f(&mut ())
        }
    }
    /*
    struct InOptionArc;

    impl<T: ?Sized + Data + Default> Lens<Option<Arc<T>>,T> for InOptionArc
    {
        fn with<V, F: FnOnce(&U) -> V>(&self, data: &Option<Arc<T>>, f: F) -> V {
            unimplemented!()
        }

        fn with_mut<V, F: FnOnce(&mut U) -> V>(&self, data: &mut Option<Arc<T>>, f: F) -> V {
            if let Some(arc) = data
            {
                let old_data : T = &*arc;
                let ret = f(arc);
                if !old_data.same(&*arc)
                {
                    *Arc::make_mut(arc) = lensed;
                    *data = Some(arc.clone());
                }
            }
        }
    }
    */
}
