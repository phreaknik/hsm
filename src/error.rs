use core::fmt;

pub enum Error {
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unimplemented => f.write_str("This function has not been implemented yet."),
        }
    }
}
