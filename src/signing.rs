use bitcoin::util::psbt::PartiallySignedTransaction;

pub enum SignatureType {
    Psbt(PartiallySignedTransaction),
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
