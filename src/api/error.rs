use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct RequestFailed {
    err_code: String,
}

impl RequestFailed {
    pub fn new(err_code: String) -> RequestFailed {
        RequestFailed { err_code }
    }
}

impl fmt::Display for RequestFailed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the request failed with status code {}", self.err_code)
    }
}

impl Error for RequestFailed {}

#[derive(Debug, Clone)]
pub struct ProjectNotFound {}

impl ProjectNotFound {
    pub fn new() -> ProjectNotFound {
        ProjectNotFound {}
    }
}

impl fmt::Display for ProjectNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the project was not found")
    }
}

impl Error for ProjectNotFound {}
