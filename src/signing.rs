use bitcoin::util::psbt::PartiallySignedTransaction;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize)]
pub enum SigType {
    Psbt,
    Message,
}

pub enum SigRequest {
    Psbt(PartiallySignedTransaction),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
