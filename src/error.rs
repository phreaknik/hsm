use bitcoin::util::bip32;
use core::convert::From;
use core::fmt;

// See fmt::Display implementation for descriptions of each error variant.
pub enum Error {
    Bip32(bip32::Error),
    KeySlotFull,
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Bip32(ref e) => fmt::Display::fmt(e, f),
            Error::KeySlotFull => f.write_str("A key already exists and cannot be overwritten."),
            Error::Unimplemented => f.write_str("This function has not been implemented yet."),
        }
    }
}

impl From<bip32::Error> for Error {
    fn from(e: bip32::Error) -> Self {
        Error::Bip32(e)
    }
}
