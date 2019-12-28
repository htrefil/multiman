use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct Error {
    pub message: &'static str,
    pub position: usize,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.position, self.message)
    }
}
