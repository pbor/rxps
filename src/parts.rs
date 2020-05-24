use std::path::PathBuf;

use crate::document::{Outline, OutlineEntry};
use crate::error::{Error, Result, XpsError};
use crate::renderer::{
    BleedBox, Brush, Canvas, Clip, ContentBox, EdgeMode, Fill, Glyphs, Indices, IsSideways,
    NavigateUri, Opacity, OpacityMask, Path, RenderNode, RenderTransform, Stroke, StrokeDashArray,
    StrokeDashOffset, StrokeEndLineCap, StrokeLineJoin, StrokeStartLineCap, StyleSimulations,
    UnicodeString,
};

/*
    FixedDocumemntSequence,
    FixedDocument,
    FixedPage,
    DocumentStructure,

Not implemented:
    Font,
    Image,
    RemoteResourceDictionary,
    Thumbnail,
    PrintTicket,
    ICCProfile,
    StoryFragment,
    SignatureDefinition,
    DiscardControl,
*/

#[rustfmt::skip]
mod ns {
    pub const XPS: &str = "http://schemas.microsoft.com/xps/2005/06";
    pub const OXPS: &str = "http://schemas.openxps.org/oxps/v1.0";
    pub const DOC_STRUCT: &str = "http://schemas.microsoft.com/xps/2005/06/documentstructure";
}

#[derive(Debug, Default)]
pub struct FixedDocumentSequence {
    // FIXME: is it correct to use Path? Or do we need something not OS dependent?
    pub(crate) sources: Vec<PathBuf>,
}

impl FixedDocumentSequence {
    pub fn parse(repr: &str) -> Result<Self> {
        let doc = roxmltree::Document::parse(repr)?;

        let mut sources = Vec::new();

        for node in doc
            .root()
            .children()
            .filter(|n| has_xps_tag_name(n, "FixedDocumentSequence"))
        {
            for node in node
                .children()
                .filter(|n| has_xps_tag_name(n, "DocumentReference"))
            {
                if let Some(source) = node.attribute("Source") {
                    sources.push(PathBuf::from(source));
                }
            }
        }

        Ok(Self { sources })
    }
}

#[derive(Debug)]
pub struct FixedDocument;

#[derive(Debug, Default)]
pub struct FixedDocumentPage {
    pub(crate) source: Option<String>,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) links: Vec<String>,
}

impl FixedDocument {
    pub fn parse(fixed_doc: &str) -> Result<Vec<FixedDocumentPage>> {
        let doc = roxmltree::Document::parse(fixed_doc)?;

        let mut pages = Vec::new();

        for node in doc
            .root()
            .children()
            .filter(|n| has_xps_tag_name(n, "FixedDocument"))
        {
            for node in node
                .children()
                .filter(|n| has_xps_tag_name(n, "PageContent"))
            {
                let mut page = FixedDocumentPage::default();

                page.source = node.attribute("Source").map(String::from);

                if let Some(w) = node.attribute("Width") {
                    page.width = parse_size(w);
                }

                if let Some(h) = node.attribute("Height") {
                    page.height = parse_size(h);
                }

                for node in node
                    .children()
                    .filter(|n| has_xps_tag_name(n, "PageContent.LinkTargets"))
                {
                    for node in node
                        .children()
                        .filter(|n| has_xps_tag_name(n, "LinkTarget"))
                    {
                        if let Some(name) = node.attribute("Name") {
                            page.links.push(String::from(name));
                        }
                    }
                }

                pages.push(page);
            }
        }

        Ok(pages)
    }
}

#[derive(Debug, Default)]
pub struct FixedPage {
    pub(crate) name: Option<String>,
    pub(crate) width: Option<f64>,
    pub(crate) height: Option<f64>,
    pub(crate) content_box: Option<ContentBox>,
    pub(crate) bleed_box: Option<BleedBox>,
    pub(crate) xml_lang: Option<String>,
    pub(crate) render_tree: RenderNode,
}

impl FixedPage {
    pub fn parse(fixed_page: &str) -> Result<Self> {
        let doc = roxmltree::Document::parse(fixed_page)?;

        let mut page = FixedPage::default();

        for node in doc
            .root()
            .children()
            .filter(|n| has_xps_tag_name(n, "FixedPage"))
        {
            page.name = node.attribute("Name").map(String::from);
            page.width = node.attribute("Width").map(parse_size);
            page.height = node.attribute("Height").map(parse_size);

            // TODO:
            // xml:lang
            // ContentBox
            // BleedBox

            parse_render_node(node, &mut page.render_tree)?;
        }

        Ok(page)
    }
}

fn parse_render_node<'a, 'i: 'a>(
    xml_node: roxmltree::Node<'a, 'i>,
    render_node: &mut RenderNode,
) -> Result<()> {
    for n in xml_node.children() {
        if has_xps_tag_name(&n, "Path") {
            let path = parse_path(n)?;
            render_node.append(RenderNode::Path(path))
        } else if has_xps_tag_name(&n, "Glyphs") {
            let glyphs = parse_glyphs(n)?;
            render_node.append(RenderNode::Glyphs(glyphs))
        } else if has_xps_tag_name(&n, "Canvas") {
            let canvas = parse_canvas(n)?;

            // Canvas is a group that contains Path, Glyphs and Canvas
            let mut canvas = RenderNode::Canvas(canvas);
            parse_render_node(n, &mut canvas)?;

            render_node.append(canvas)
        }
    }

    Ok(())
}

fn parse_canvas<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<Canvas> {
    let mut canvas = Canvas::default();

    canvas.name = node.attribute("Name").map(String::from);

    // FIXME: xml:lang - how does roxmltree deal with attribute namespaces?

    // FIXME: x:Key - how does roxmltree deal with attribute namespaces?

    canvas.render_transform = node
        .attribute("RenderTransform")
        .and_then(|s| s.parse::<RenderTransform>().ok());

    canvas.clip = node.attribute("Clip").and_then(|s| s.parse::<Clip>().ok());

    canvas.opacity = node
        .attribute("Opacity")
        .and_then(|s| s.parse::<Opacity>().ok());

    canvas.opacity_mask = node
        .attribute("OpacityMask")
        .and_then(|s| s.parse::<OpacityMask>().ok());

    canvas.edge_mode = node
        .attribute("RenderOptions.EdgeMode")
        .and_then(|s| s.parse::<EdgeMode>().ok());

    canvas.navigate_uri = node
        .attribute("FixedPage.NavigateUri")
        .and_then(|s| s.parse::<NavigateUri>().ok());

    // TODO:
    // AutomationProperties.Name
    // AutomationProperties.HelpText

    for n in node.children() {
        if has_xps_tag_name(&n, "Canvas.Resources") {
            parse_resources(n)?;
        } else if has_xps_tag_name(&n, "Canvas.RenderTransform") {
            parse_render_transform(n)?;
        } else if has_xps_tag_name(&n, "Canvas.Clip") {
            parse_clip(n)?;
        } else if has_xps_tag_name(&n, "Canvas.OpacityMask") {
            parse_opacity_mask(n)?;
        }
    }

    Ok(canvas)
}

fn parse_glyphs<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<Glyphs> {
    let mut glyphs = Glyphs::default();

    glyphs.name = node.attribute("Name").map(String::from);

    // FIXME: xml:lang - how does roxmltree deal with attribute namespaces?

    // FIXME: x:Key - how does roxmltree deal with attribute namespaces?

    glyphs.origin.0 = node
        .attribute("OriginX")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or_default();
    glyphs.origin.1 = node
        .attribute("OriginY")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or_default();

    if let Some(u) = node.attribute("FontUri") {
        glyphs.font_uri = String::from(u);
    }

    if let Some(s) = node.attribute("FontRenderingEmSize") {
        glyphs.font_rendering_em_size = parse_size(s);
    }

    /* TODO:

    glyphs.bidi_level = node
        .attribute("BidiLevel")
        .and_then(|s| s.parse::<BidiLevel>().ok());

    glyphs.caret_stops = node
        .attribute("CaretStops")
        .and_then(|s| s.parse::<CaretStops>().ok());

    glyphs.device_font_name = node
        .attribute("DeviceFontName")
        .and_then(|s| s.parse::<DeviceFontName>().ok());

    */

    glyphs.fill = node.attribute("Fill").and_then(|s| s.parse::<Fill>().ok());

    glyphs.is_sideways = node
        .attribute("IsSideways")
        .and_then(|s| s.parse::<IsSideways>().ok());

    glyphs.indices = node
        .attribute("Indices")
        .and_then(|s| s.parse::<Indices>().ok());

    glyphs.unicode_string = node
        .attribute("UnicodeString")
        .and_then(|s| s.parse::<UnicodeString>().ok());

    glyphs.style_simulations = node
        .attribute("StyleSimulations")
        .and_then(|s| s.parse::<StyleSimulations>().ok());

    glyphs.render_transform = node
        .attribute("RenderTransform")
        .and_then(|s| s.parse::<RenderTransform>().ok());

    glyphs.clip = node.attribute("Clip").and_then(|s| s.parse::<Clip>().ok());

    glyphs.opacity = node
        .attribute("Opacity")
        .and_then(|s| s.parse::<Opacity>().ok());

    glyphs.opacity_mask = node
        .attribute("OpacityMask")
        .and_then(|s| s.parse::<OpacityMask>().ok());

    glyphs.navigate_uri = node
        .attribute("FixedPage.NavigateUri")
        .and_then(|s| s.parse::<NavigateUri>().ok());

    for n in node.children() {
        if has_xps_tag_name(&n, "Glyphs.RenderTransform") {
            parse_render_transform(n)?;
        } else if has_xps_tag_name(&n, "Glyps.Clip") {
            parse_clip(n)?;
        } else if has_xps_tag_name(&n, "Glyps.Fill") {
            parse_fill(n)?;
        } else if has_xps_tag_name(&n, "Glyps.OpacityMask") {
            parse_opacity_mask(n)?;
        }
    }

    Ok(glyphs)
}

fn parse_path<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<Path> {
    let mut path = Path::default();

    path.name = node.attribute("Name").map(String::from);

    // FIXME: xml:lang - how does roxmltree deal with attribute namespaces?

    // FIXME: x:Key - how does roxmltree deal with attribute namespaces?

    path.data = node.attribute("Data").map(String::from);

    path.fill = node.attribute("Fill").and_then(|s| s.parse::<Fill>().ok());

    path.render_transform = node
        .attribute("RenderTransform")
        .and_then(|s| s.parse::<RenderTransform>().ok());

    path.clip = node.attribute("Clip").and_then(|s| s.parse::<Clip>().ok());

    path.opacity = node
        .attribute("Opacity")
        .and_then(|s| s.parse::<Opacity>().ok());

    path.opacity_mask = node
        .attribute("OpacityMask")
        .and_then(|s| s.parse::<OpacityMask>().ok());

    path.stroke = node
        .attribute("Stroke")
        .and_then(|s| s.parse::<Stroke>().ok());

    path.stroke_dash_array = node
        .attribute("StrokeDashArray")
        .and_then(|s| s.parse::<StrokeDashArray>().ok());

    path.stroke_dash_offset = node
        .attribute("StrokeDashOffset")
        .and_then(|s| s.parse::<StrokeDashOffset>().ok());

    path.stroke_start_line_cap = node
        .attribute("StrokeStartLineCap")
        .and_then(|s| s.parse::<StrokeStartLineCap>().ok());

    path.stroke_end_line_cap = node
        .attribute("StrokeEndLineCap")
        .and_then(|s| s.parse::<StrokeEndLineCap>().ok());

    path.stroke_line_join = node
        .attribute("StrokeLineJoin")
        .and_then(|s| s.parse::<StrokeLineJoin>().ok());

    for n in node.children() {
        if has_xps_tag_name(&n, "Path.Data") {
            parse_path_data(n)?;
        } else if has_xps_tag_name(&n, "Path.RenderTransform") {
            parse_render_transform(n)?;
        } else if has_xps_tag_name(&n, "Path.Clip") {
            parse_clip(n)?;
        } else if has_xps_tag_name(&n, "Path.Fill") {
            parse_fill(n)?;
        } else if has_xps_tag_name(&n, "Path.Stroke") {
            parse_stroke(n)?;
        } else if has_xps_tag_name(&n, "Path.OpacityMask") {
            parse_opacity_mask(n)?;
        }
    }

    Ok(path)
}

fn parse_render_transform<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_clip<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_fill<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<()> {
    let _brush = parse_brush(node)?;

    // TODO
    Ok(())
}

fn parse_stroke<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<()> {
    let _brush = parse_brush(node)?;

    // TODO
    Ok(())
}

fn parse_opacity_mask<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<()> {
    let _brush = parse_brush(node)?;

    // TODO
    Ok(())
}

fn parse_brush<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<Brush> {
    for n in node.children() {
        if has_xps_tag_name(&n, "ImageBrush") {
            parse_image_brush(n)?;
            return Ok(Brush::Image);
        } else if has_xps_tag_name(&n, "LinearGradientBrush") {
            parse_linear_gradient_brush(n)?;
            return Ok(Brush::LinearGradient);
        } else if has_xps_tag_name(&n, "RadialGradientBrush") {
            parse_radial_gradient_brush(n)?;
            return Ok(Brush::RadialGradient);
        } else if has_xps_tag_name(&n, "SolidColorBrush") {
            parse_solid_color_brush(n)?;
            return Ok(Brush::SolidColor);
        } else if has_xps_tag_name(&n, "VisualBrush") {
            parse_visual_brush(n)?;
            return Ok(Brush::Visual);
        }
    }

    Err(Error::Xps(XpsError::MissingBrush))
}

fn parse_image_brush<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_linear_gradient_brush<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_radial_gradient_brush<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_solid_color_brush<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_visual_brush<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_resources<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

fn parse_path_data<'a, 'i: 'a>(_node: roxmltree::Node<'a, 'i>) -> Result<()> {
    // TODO
    Ok(())
}

#[derive(Debug, Default)]
pub struct DocumentStructure {
    pub(crate) outline: Option<Outline>,
}

impl DocumentStructure {
    pub fn parse(structure: &str) -> Result<DocumentStructure> {
        let doc = roxmltree::Document::parse(structure)?;

        let mut res = DocumentStructure::default();

        for node in doc
            .root()
            .children()
            .filter(|n| n.has_tag_name((ns::DOC_STRUCT, "DocumentStructure")))
        {
            for node in node
                .children()
                .filter(|n| n.has_tag_name((ns::DOC_STRUCT, "DocumentStructure.Outline")))
            {
                for node in node
                    .children()
                    .filter(|n| n.has_tag_name((ns::DOC_STRUCT, "DocumentOutline")))
                {
                    let mut outline = Outline::default();

                    for node in node
                        .children()
                        .filter(|n| n.has_tag_name((ns::DOC_STRUCT, "OutlineEntry")))
                    {
                        let mut entry = OutlineEntry::default();

                        entry.level = node.attribute("OutlineLevel").map(String::from);
                        entry.description = node.attribute("Description").map(String::from);
                        entry.target = node.attribute("OutlineTarget").map(String::from);

                        outline.entries.push(entry);
                    }

                    res.outline = Some(outline);
                }
            }
        }

        Ok(res)
    }
}

fn has_xps_tag_name(node: &roxmltree::Node<'_, '_>, tag: &str) -> bool {
    node.has_tag_name((ns::XPS, tag)) || node.has_tag_name((ns::OXPS, tag))
}

fn parse_size(s: &str) -> f64 {
    s.parse().ok().filter(|&f| f >= 0.0).unwrap_or_default()
}
