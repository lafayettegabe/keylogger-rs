use std::fmt;

#[derive(Debug)]
pub enum OsError {
    UnsupportedOS,
    UnsupportedFeature,
    IoError(std::io::Error),
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OsError::UnsupportedOS => write!(f, "Unsupported operating system"),
            OsError::UnsupportedFeature => write!(f, "Feature not compiled in"),
            OsError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<std::io::Error> for OsError {
    fn from(err: std::io::Error) -> Self {
        OsError::IoError(err)
    }
}

impl std::error::Error for OsError {}
