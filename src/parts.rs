use std::path::PathBuf;

use crate::document::{Outline, OutlineEntry};
use crate::error::Result;
use crate::page;

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
    pub(crate) content_box: Option<page::ContentBox>,
    pub(crate) bleed_box: Option<page::BleedBox>,
    pub(crate) xml_lang: Option<String>,
    pub(crate) render_tree: page::RenderNode,
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
    render_node: &mut page::RenderNode,
) -> Result<()> {
    for n in xml_node.children() {
        if has_xps_tag_name(&n, "Path") {
            let path = parse_path(n)?;
            render_node.append(page::RenderNode::Path(path))
        } else if has_xps_tag_name(&n, "Glyphs") {
            let glyphs = parse_glyphs(n)?;
            render_node.append(page::RenderNode::Glyphs(glyphs))
        } else if has_xps_tag_name(&n, "Canvas") {
            let canvas = parse_canvas(n)?;

            // Canvas is a group that contains Path, Glyphs and Canvas
            let mut canvas = page::RenderNode::Canvas(canvas);
            parse_render_node(n, &mut canvas)?;

            render_node.append(canvas)
        }
    }

    Ok(())
}

fn parse_path<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<page::Path> {
    let path = page::Path::default();

    // TODO

    for n in node.children() {
        if has_xps_tag_name(&n, "Path.Data") {
        } else if has_xps_tag_name(&n, "Path.RenderTransform") {
        } else if has_xps_tag_name(&n, "Path.Clip") {
        } else if has_xps_tag_name(&n, "Path.Fill") {
        } else if has_xps_tag_name(&n, "Path.Stroke") {
        } else if has_xps_tag_name(&n, "Path.OpacityMask") {
        }
    }

    Ok(path)
}

fn parse_glyphs<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<page::Glyphs> {
    let mut glyphs = page::Glyphs::default();

    glyphs.name = node.attribute("Name").map(String::from);

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

    // TODO:
    // BidiLevel,
    // CaretStops,
    // DevideFontName,
    // Fill,
    // IsSideways,
    // Indices,
    // UnicodeString,
    // StyleSimulations,
    // RenderTransform,
    // Clip,
    // Opacity,
    // OpacityMask,
    // FixedPage.NavigateUri,
    // xml:lang,
    // x:Key,

    for n in node.children() {
        if has_xps_tag_name(&n, "Glyphs.RenderTransform") {
        } else if has_xps_tag_name(&n, "Glyps.Clip") {
        } else if has_xps_tag_name(&n, "Glyps.Fill") {
        } else if has_xps_tag_name(&n, "Glyps.OpacityMask") {
        }
    }

    Ok(glyphs)
}

fn parse_canvas<'a, 'i: 'a>(node: roxmltree::Node<'a, 'i>) -> Result<page::Canvas> {
    let canvas = page::Canvas::default();

    // TODO

    for n in node.children() {
        if has_xps_tag_name(&n, "Canvas.Resources") {
        } else if has_xps_tag_name(&n, "Canvas.RenderTransform") {
        } else if has_xps_tag_name(&n, "Canvas.Clip") {
        } else if has_xps_tag_name(&n, "Canvas.OpacityMask") {
        }
    }

    Ok(canvas)
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
