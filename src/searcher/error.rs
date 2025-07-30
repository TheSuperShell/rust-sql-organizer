use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    EmptyFileExtension,
    GlobPatternError(glob::PatternError),
    GlobError(glob::GlobError),
}

impl From<glob::GlobError> for Error {
    fn from(value: glob::GlobError) -> Self {
        Self::GlobError(value)
    }
}

impl From<glob::PatternError> for Error {
    fn from(value: glob::PatternError) -> Self {
        Self::GlobPatternError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyFileExtension => f.write_str("Extension is empty"),
            Self::GlobPatternError(e) => e.fmt(f),
            Self::GlobError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
