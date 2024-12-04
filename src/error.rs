use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    FileNotFound(io::Error),
    SolutionNotExists,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::FileNotFound(e) => write!(f, "File read failed with: {e}"),
            Error::SolutionNotExists => write!(f, "Solution not yet implemented."),
        }
    }
}
