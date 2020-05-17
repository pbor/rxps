use crate::page::Page;

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

    /// Returns an iterator that yields `Pages` in the document.
    pub fn pages(&self) -> Pages<'_> {
        Pages(self.pages.iter())
    }
}

/// Iterator from `Document.pages`.
#[derive(Debug)]
pub struct Pages<'a>(std::slice::Iter<'a, Page>);

impl<'a> Iterator for Pages<'a> {
    type Item = &'a Page;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// Outline of a `Document`. The outline of a document is a tree
/// of (description, link) pairs that forms a table of contents.
#[derive(Debug, Default)]
pub struct Outline {
    pub(crate) entries: Vec<OutlineEntry>,
}

impl Outline {
    /// Returns an iterator that yields `OutlineEntries` in the document outline.
    pub fn entries(&self) -> OutlineEntries<'_> {
        OutlineEntries(self.entries.iter())
    }
}

/// Iterator from `Outline.entries`.
#[derive(Debug)]
pub struct OutlineEntries<'a>(std::slice::Iter<'a, OutlineEntry>);

impl<'a> Iterator for OutlineEntries<'a> {
    type Item = &'a OutlineEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug, Default)]
pub struct OutlineEntry {
    pub(crate) level: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) target: Option<String>,
}
