use crate::error::Result;

#[rustfmt::skip]
mod ns {
    pub const RELS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
}

#[rustfmt::skip]
mod ty {
    pub const CORE_PROPERTIES: &str = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";
    pub const THUMBNAIL: &str = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
    pub const FIXED_REPRESENTATION: &str = "http://schemas.microsoft.com/xps/2005/06/fixedrepresentation";
    pub const OXPS_FIXED_REPRESENTATION: &str = "http://schemas.openxps.org/oxps/v1.0/fixedrepresentation";
    pub const DOCUMENT_STRUCTURE: &str = "http://schemas.microsoft.com/xps/2005/06/documentstructure";
}

#[derive(Debug, Default)]
pub struct PackageRelationships {
    pub core_properties: Option<String>,
    pub thumbnail: Option<String>,
    pub fixed_representation: Option<String>,
}

impl PackageRelationships {
    pub fn parse(rels: &str) -> Result<Self> {
        let doc = roxmltree::Document::parse(rels)?;

        let mut res = Self::default();

        for node in doc
            .root()
            .children()
            .filter(|n| n.has_tag_name((ns::RELS, "Relationships")))
        {
            for node in node
                .children()
                .filter(|n| n.has_tag_name((ns::RELS, "Relationship")))
            {
                if let Some(ty) = node.attribute("Type") {
                    let target = node.attribute("Target").map(String::from);

                    match ty {
                        ty::CORE_PROPERTIES => res.core_properties = target,
                        ty::THUMBNAIL => res.thumbnail = target,
                        ty::FIXED_REPRESENTATION | ty::OXPS_FIXED_REPRESENTATION => {
                            res.fixed_representation = target
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Default)]
pub struct DocumentRelationships {
    pub structure: Option<String>,
}

impl DocumentRelationships {
    pub fn parse(rels: &str) -> Result<Self> {
        let doc = roxmltree::Document::parse(rels)?;

        let mut res = Self::default();

        for node in doc
            .root()
            .children()
            .filter(|n| n.has_tag_name((ns::RELS, "Relationships")))
        {
            for node in node
                .children()
                .filter(|n| n.has_tag_name((ns::RELS, "Relationship")))
            {
                if let Some(ty) = node.attribute("Type") {
                    let target = node.attribute("Target").map(String::from);

                    if ty == ty::DOCUMENT_STRUCTURE {
                        res.structure = target
                    }
                }
            }
        }

        Ok(res)
    }
}
