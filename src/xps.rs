use log::debug;
use std::path::Path;

use crate::archive::Archive;
use crate::error::Result;
use crate::parts::{DocumentStructure, FixedDocument, FixedDocumentSequence, FixedPage};
use crate::relationships::{DocumentRelationships, PackageRelationships};
use crate::renderer::Renderer;

/// The main XPS entry point
#[derive(Debug)]
pub struct XPS {
    documents: Vec<Document>,
}

impl XPS {
    /// Loads an XPS archive
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        debug!("Loading XPS {}", path.as_ref().display());

        // TODO: this loads and parses all parts. Loading everything
        // upfront, including all the pages, is *very* slow on a
        // large document when running a debug build, but it seems
        // "fast enough" in release builds...
        // Should we consider loading page content lazily? That opens
        // the can of worms related to shared mutable access to the
        // archive.

        let mut archive = Archive::new(path)?;

        let rels = archive
            .get_as_string("_rels/.rels")
            .and_then(|s| PackageRelationships::parse(&s))?;

        debug!("Package Relationships {:?}", rels);

        let mut documents = Vec::new();

        if let Some(fixed_repr) = &rels.fixed_representation {
            let fr = archive
                .get_as_string(fixed_repr)
                .and_then(|s| FixedDocumentSequence::parse(&s))?;

            for s in fr.sources.iter().filter_map(|f| f.strip_prefix("/").ok()) {
                let mut doc = Document::default();

                if let Some(n) = s.file_name() {
                    let mut rels_name = n.to_os_string();
                    rels_name.push(".rels");
                    let mut path = s.to_path_buf();
                    path.pop();
                    path.push("_rels");
                    path.push(rels_name);

                    let doc_rels = archive
                        .get_as_string(path)
                        .and_then(|s| DocumentRelationships::parse(&s))?;

                    if let Some(ref structure) = doc_rels.structure {
                        let mut path = s.to_path_buf();
                        path.pop();
                        path.push(structure);

                        let doc_structure = archive
                            .get_as_string(path)
                            .and_then(|s| DocumentStructure::parse(&s))?;

                        doc.outline = doc_structure.outline;
                    }
                }

                let pages = archive
                    .get_as_string(s)
                    .and_then(|s| FixedDocument::parse(&s))?;

                for p in pages.into_iter().filter(|p| p.source.is_some()) {
                    let source = p.source.as_ref().unwrap();
                    let mut path = s.to_path_buf();
                    path.pop();
                    path.push(source);

                    let fixed_page = archive
                        .get_as_string(path)
                        .and_then(|s| FixedPage::parse(&s))?;

                    doc.pages.push(Page {
                        width: fixed_page.width.unwrap_or(p.width),
                        height: fixed_page.height.unwrap_or(p.height),
                        name: fixed_page.name,
                        render_tree: fixed_page.render_tree,
                        links: p.links,
                    });
                }

                documents.push(doc);
            }
        }

        debug!("Documents {:?}", documents);

        Ok(Self { documents })
    }

    /// Returns the `Documents` in the XPS archive.
    pub fn documents(&self) -> &[Document] {
        &self.documents
    }
}

/// A document inside the XPS archive
#[derive(Debug, Default)]
pub struct Document {
    pub(crate) outline: Option<Outline>,
    pub(crate) pages: Vec<Page>,
}

impl Document {
    /// Returns the outline of the document, if available.
    pub fn outline(&self) -> Option<&Outline> {
        self.outline.as_ref()
    }

    /// Returns the `Pages` in the document.
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
}

/// Outline of a `Document`. The outline of a document is a tree
/// of (description, link) pairs that forms a table of contents.
#[derive(Debug, Default)]
pub struct Outline {
    pub(crate) entries: Vec<OutlineEntry>,
}

impl Outline {
    /// Returns the `OutlineEntries` in the document outline.
    pub fn entries(&self) -> &[OutlineEntry] {
        &self.entries
    }
}

/// An entry in the `Outline` of a `Document`
#[derive(Debug, Default)]
pub struct OutlineEntry {
    pub(crate) level: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) target: Option<String>,
}

use crate::renderer::RenderNode;

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

    /// Renders a page with the given `Renderer`
    pub fn render(&self, renderer: &impl Renderer) -> Result<()> {
        renderer.render(&self.render_tree)?;

        Ok(())
    }
}
