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
}
