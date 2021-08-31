use std::error::Error;

#[derive(Clone, Debug)]
pub struct GenericError {
    pub message: String,
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for GenericError {}
