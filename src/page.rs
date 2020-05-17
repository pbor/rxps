/// A page in a `Document`
#[derive(Debug)]
pub struct Page {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) name: Option<String>,
    pub(crate) render_tree: RenderNode,
    pub(crate) links: Vec<String>,
}

impl Page {
    /// Returns the size of the page
    pub fn size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    /// Renders this page
    pub fn render(&self) {
        dbg!(&self.render_tree);
    }
}

#[derive(Debug, Default)]
pub(crate) struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Default)]
pub(crate) struct ContentBox(Rect);

#[derive(Debug, Default)]
pub(crate) struct BleedBox(Rect);

#[derive(Debug, Default)]
pub(crate) struct Opacity(f64);

#[derive(Debug, Default)]
pub(crate) struct RenderTransform(f64, f64, f64, f64, f64, f64);

#[derive(Debug, Default)]
pub(crate) struct Clip {}

#[derive(Debug, Default)]
pub(crate) struct Fill {}

#[derive(Debug, Default)]
pub(crate) struct Stroke {}

#[derive(Debug, Default)]
pub(crate) struct StrokeDashArray {}

#[derive(Debug, Default)]
pub(crate) struct Path {
    pub(crate) data: String,
    pub(crate) name: Option<String>,
    pub(crate) xml_lang: Option<String>,
    pub(crate) render_transform: RenderTransform,
    pub(crate) clip: Clip,
    pub(crate) fill: Fill,
    pub(crate) stroke: Stroke,
    pub(crate) stroke_dash_array: StrokeDashArray,
    pub(crate) stroke_dash_offset: f64,
    pub(crate) stroke_end_line_cap: f64,
    pub(crate) stroke_start_line_cap: f64,
    pub(crate) stroke_start_line_join: f64,
    pub(crate) stroke_miter_limit: f64,
    pub(crate) stroke_thickness: f64,
    pub(crate) opacity: Opacity,
    pub(crate) navigate_uri: Option<String>,
}

#[derive(Debug, Default)]
pub(crate) struct Glyphs {
    pub(crate) name: Option<String>,
    pub(crate) xml_lang: Option<String>,
    pub(crate) origin: (f64, f64),
    pub(crate) render_transform: RenderTransform,
    pub(crate) opacity: Opacity,
    pub(crate) unicode_string: String,
    pub(crate) font_uri: String, // FIXME: use a type for URI
    pub(crate) font_rendering_em_size: f64,
}

#[derive(Debug, Default)]
pub(crate) struct Canvas {
    pub(crate) render_transform: RenderTransform,
    pub(crate) opacity: Opacity,
    pub(crate) children: Vec<RenderNode>,
}

#[derive(Debug)]
pub(crate) enum RenderNode {
    Root(Vec<RenderNode>),
    Path(Path),
    Glyphs(Glyphs),
    Canvas(Canvas),
}

impl RenderNode {
    pub fn append(&mut self, node: RenderNode) {
        match self {
            Self::Root(v) => v.push(node),
            Self::Canvas(c) => c.children.push(node),
            _ => unreachable!(),
        }
    }
}

impl Default for RenderNode {
    fn default() -> Self {
        Self::Root(Vec::new())
    }
}
