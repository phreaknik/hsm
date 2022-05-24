use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use serde::{Deserialize, Serialize};

/// Types of signatures that can be performed
#[derive(PartialEq, Deserialize, Serialize)]
pub enum SigType {
    Psbt,
    Message,
}

/// Request for signature
pub enum SigRequest {
    Psbt(Psbt),
    Message(String),
}

impl SigRequest {
    pub fn sigtype(&self) -> SigType {
        match &self {
            SigRequest::Psbt(_) => SigType::Psbt,
            SigRequest::Message(_) => SigType::Message,
        }
    }
}

/// Witness data output from the signing process
pub enum SignedData {
    Psbt(Psbt),
    Message(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
