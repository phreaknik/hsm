use bitcoin::secp256k1;
use bitcoin::util::bip32;
use core::convert::From;
use core::fmt;

// See fmt::Display implementation for descriptions of each error variant.
pub enum Error {
    Bip32(bip32::Error),
    Secp256k1(secp256k1::Error),
    NoPrivKey,
    NotFound,
    DuplicateEntry,
    KeySlotFull,
    MissingNonWitnessUtxo,
    NonStandardSighash,
    UnsupportedUtxoType,
    Unimplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Bip32(ref e) => fmt::Display::fmt(e, f),
            Error::Secp256k1(ref e) => fmt::Display::fmt(e, f),
            Error::NoPrivKey => f.write_str("No private key!"),
            Error::NotFound => f.write_str("Entry was not found."),
            Error::DuplicateEntry => f.write_str("Entry already exists and may not be duplicated."),
            Error::KeySlotFull => f.write_str("A key already exists and cannot be overwritten."),
            Error::MissingNonWitnessUtxo => f.write_str(
                "The non_witness_utxo field of the transaction is required to sign this input.",
            ),
            Error::NonStandardSighash => {
                f.write_str("The psbt contains a non-'SIGHASH_ALL' sighash in one of its inputs.")
            }
            Error::UnsupportedUtxoType => f.write_str("Unknown UTXO type found."),
            Error::Unimplemented => f.write_str("This function has not been implemented yet."),
        }
    }
}

impl From<bip32::Error> for Error {
    fn from(e: bip32::Error) -> Self {
        Error::Bip32(e)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Self {
        Error::Secp256k1(e)
    }
}
