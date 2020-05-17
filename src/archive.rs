use log::debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Result;

#[derive(Debug)]
pub struct Archive {
    zip: zip::ZipArchive<File>,
}

impl Archive {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref())?;

        Ok(Archive {
            zip: zip::ZipArchive::new(file)?,
        })
    }

    // FIXME: does this need to be case insensitive?
    pub fn get<P: AsRef<Path>>(&mut self, name: P) -> Result<Vec<u8>> {
        let name = name.as_ref().display().to_string();

        debug!("Getting archive item {}", name);

        let mut res: Vec<u8> = vec![];
        self.zip.by_name(&name)?.read_to_end(&mut res)?;

        Ok(res)
    }

    pub fn get_as_string<P: AsRef<Path>>(&mut self, name: P) -> Result<String> {
        let bytes = self.get(name)?;

        // According to the spec, XML files in XPS can only be UTF-8 or UTF-16.
        // Here I only manually check if the UTF-18 LE BOM and if present collect
        // into a Vec<u16> and convert from_utf16. Otherwise assume it is UTF-8.
        let mut iter = bytes.chunks(2).map(|c| c[0] as u16 | ((c[1] as u16) << 8));
        let bom = iter.by_ref().peekable().next();

        let res = if let Some(0xfeff) = bom {
            String::from_utf16(&iter.collect::<Vec<u16>>())?
        } else {
            String::from_utf8(bytes)?
        };

        Ok(res)
    }
}
