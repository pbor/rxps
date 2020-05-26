pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    MissingBrush,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingBrush => write!(f, "Missing brush element"),
        }
    }
}

impl std::error::Error for ParseError {}

pub type RenderResult<T> = std::result::Result<T, RenderError>;

#[derive(Debug)]
pub enum RenderError {
    #[cfg(feature = "cairo-renderer")]
    Cairo, // FIXME: wrap the cairo error

    Unknown,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "cairo-renderer")]
            RenderError::Cairo => write!(f, "Cairo error"),
            RenderError::Unknown => write!(f, "Unknown rendering error"),
        }
    }
}

impl std::error::Error for RenderError {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Utf16(std::string::FromUtf16Error),
    Zip(zip::result::ZipError),
    Xml(roxmltree::Error),
    Xps(ParseError),
    Render(RenderError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => e.fmt(f),
            Error::Zip(e) => e.fmt(f),
            Error::Utf8(e) => e.fmt(f),
            Error::Utf16(e) => e.fmt(f),
            Error::Xml(e) => e.fmt(f),
            Error::Xps(e) => e.fmt(f),
            Error::Render(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(err: std::string::FromUtf16Error) -> Self {
        Error::Utf16(err)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(err)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(err: roxmltree::Error) -> Self {
        Error::Xml(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::Xps(err)
    }
}

impl From<RenderError> for Error {
    fn from(err: RenderError) -> Self {
        Error::Render(err)
    }
}
