use std::str::FromStr;

use crate::error::RenderResult;

/// Renderer trait
pub trait Renderer {
    /// Renders a tree of `RenderNode`s
    fn render(&self, tree: &RenderNode) -> RenderResult<()> {
        match tree {
            RenderNode::Root(ref children) => self.render_children(&children),
            _ => unreachable!(),
        }
    }

    /// Renders children of a node
    fn render_children(&self, children: &[RenderNode]) -> RenderResult<()> {
        for c in children {
            match c {
                RenderNode::Path(p) => self.render_path(&p)?,
                RenderNode::Glyphs(g) => self.render_glyphs(&g)?,
                RenderNode::Canvas(c) => self.render_canvas(&c)?,
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    /// Renders a canvas node
    fn render_canvas(&self, canvas: &Canvas) -> RenderResult<()>;

    /// Renders a glyphs node
    fn render_glyphs(&self, glyphs: &Glyphs) -> RenderResult<()>;

    /// Renders a path node
    fn render_path(&self, path: &Path) -> RenderResult<()>;
}

#[derive(Debug)]
pub enum RenderNode {
    Root(Vec<RenderNode>),
    Canvas(Canvas),
    Glyphs(Glyphs),
    Path(Path),
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

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct ContentBox(Rect);

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct BleedBox(Rect);

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct RenderTransform {
    pub xx: f64,
    pub yx: f64,
    pub xy: f64,
    pub yy: f64,
    pub x0: f64,
    pub y0: f64,
}

impl FromStr for RenderTransform {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split(',').collect();

        if v.len() != 6 {
            return Err(());
        }

        let xx = v[0].parse::<f64>().map_err(|_| ())?;
        let yx = v[1].parse::<f64>().map_err(|_| ())?;
        let xy = v[2].parse::<f64>().map_err(|_| ())?;
        let yy = v[3].parse::<f64>().map_err(|_| ())?;
        let x0 = v[4].parse::<f64>().map_err(|_| ())?;
        let y0 = v[5].parse::<f64>().map_err(|_| ())?;

        Ok(Self {
            xx,
            yx,
            xy,
            yy,
            x0,
            y0,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Clip {}

impl FromStr for Clip {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Opacity(f64);

impl FromStr for Opacity {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f64>().map(Self).map_err(|_| ())
    }
}

#[derive(Debug, Default)]
pub(crate) struct OpacityMask {}

impl FromStr for OpacityMask {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct Fill {}

impl FromStr for Fill {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct Stroke {}

impl FromStr for Stroke {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeDashArray {}

impl FromStr for StrokeDashArray {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeDashOffset {}

impl FromStr for StrokeDashOffset {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeEndLineCap {}

impl FromStr for StrokeEndLineCap {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeStartLineCap {}

impl FromStr for StrokeStartLineCap {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeLineJoin {}

impl FromStr for StrokeLineJoin {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct StrokeMiterLimit {}

#[derive(Debug, Default)]
pub(crate) struct StrokeThickness {}

#[derive(Debug, Default)]
pub(crate) struct IsSideways(bool);

impl FromStr for IsSideways {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bool>().map(Self).map_err(|_| ())
    }
}

#[derive(Debug, Default)]
pub(crate) struct Indices(bool);

impl FromStr for Indices {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bool>().map(Self).map_err(|_| ())
    }
}

#[derive(Debug, Default)]
pub(crate) struct UnicodeString(bool);

impl FromStr for UnicodeString {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bool>().map(Self).map_err(|_| ())
    }
}

#[derive(Debug, Default)]
pub(crate) struct StyleSimulations(bool);

impl FromStr for StyleSimulations {
    type Err = (); // FIXME

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<bool>().map(Self).map_err(|_| ())
    }
}

#[derive(Debug, Default)]
pub(crate) struct EdgeMode {}

impl FromStr for EdgeMode {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug, Default)]
pub(crate) struct NavigateUri {}

impl FromStr for NavigateUri {
    type Err = (); // FIXME

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Self {})
    }
}

#[derive(Debug)]
pub(crate) enum Brush {
    Image,
    LinearGradient,
    RadialGradient,
    SolidColor,
    Visual,
}

#[derive(Debug, Default)]
pub struct Canvas {
    // common properties
    pub(crate) name: Option<String>,
    pub(crate) xml_lang: Option<String>,

    // properties
    pub(crate) render_transform: Option<RenderTransform>,
    pub(crate) clip: Option<Clip>,
    pub(crate) opacity: Option<Opacity>,
    pub(crate) opacity_mask: Option<OpacityMask>,
    pub(crate) edge_mode: Option<EdgeMode>,
    pub(crate) navigate_uri: Option<NavigateUri>,

    // content
    pub(crate) children: Vec<RenderNode>,
}

#[derive(Debug, Default)]
pub struct Glyphs {
    // common properties
    pub(crate) name: Option<String>,
    pub(crate) xml_lang: Option<String>,

    // mandatory properties
    pub(crate) origin: (f64, f64),
    pub(crate) font_uri: String, // FIXME: use a type for URI
    pub(crate) font_rendering_em_size: f64,

    // properties
    pub(crate) fill: Option<Fill>,
    pub(crate) render_transform: Option<RenderTransform>,
    pub(crate) clip: Option<Clip>,
    pub(crate) opacity: Option<Opacity>,
    pub(crate) opacity_mask: Option<OpacityMask>,
    pub(crate) is_sideways: Option<IsSideways>,
    pub(crate) indices: Option<Indices>,
    pub(crate) unicode_string: Option<UnicodeString>,
    pub(crate) style_simulations: Option<StyleSimulations>,
    pub(crate) edge_mode: Option<EdgeMode>,
    pub(crate) navigate_uri: Option<NavigateUri>,
}

#[derive(Debug, Default)]
pub struct Path {
    // common properties
    pub(crate) name: Option<String>,
    pub(crate) xml_lang: Option<String>,

    // properties
    pub(crate) fill: Option<Fill>,
    pub(crate) render_transform: Option<RenderTransform>,
    pub(crate) clip: Option<Clip>,
    pub(crate) opacity: Option<Opacity>,
    pub(crate) opacity_mask: Option<OpacityMask>,
    pub(crate) stroke: Option<Stroke>,
    pub(crate) stroke_dash_array: Option<StrokeDashArray>,
    pub(crate) stroke_dash_offset: Option<StrokeDashOffset>,
    pub(crate) stroke_end_line_cap: Option<StrokeEndLineCap>,
    pub(crate) stroke_start_line_cap: Option<StrokeStartLineCap>,
    pub(crate) stroke_line_join: Option<StrokeLineJoin>,
    pub(crate) stroke_miter_limit: Option<StrokeMiterLimit>,
    pub(crate) stroke_thickness: Option<StrokeThickness>,
    pub(crate) navigate_uri: Option<String>,

    // content
    pub(crate) data: Option<String>,
}
