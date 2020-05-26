use crate::renderer::{Canvas, Glyphs, Path, RenderTransform, Renderer};

use crate::error::RenderResult;

/// Cairo renderer
#[derive(Debug)]
pub struct CairoRenderer {
    cr: cairo::Context,
}

impl CairoRenderer {
    /// Returns a renderer for the given cairo context
    pub fn new(cr: cairo::Context) -> Self {
        Self { cr }
    }
}

impl Renderer for CairoRenderer {
    fn render_canvas(&self, _canvas: &Canvas) -> RenderResult<()> {
        Ok(())
    }

    fn render_glyphs(&self, _glyphs: &Glyphs) -> RenderResult<()> {
        Ok(())
    }

    fn render_path(&self, path: &Path) -> RenderResult<()> {
        if let Some(t) = path.render_transform {
            self.cr.transform(t.into())
        }

        Ok(())
    }
}

impl From<RenderTransform> for cairo::Matrix {
    #[inline]
    fn from(t: RenderTransform) -> Self {
        Self::new(t.xx, t.yx, t.xy, t.yy, t.x0, t.y0)
    }
}
