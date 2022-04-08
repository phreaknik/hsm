use crate::signing::SignatureType;

pub struct Policy {
    // Signature type this policy applies to
    sigtype: SignatureType,
}

impl Policy {
    pub fn is_valid(&self) -> bool {
        false // Unimplemented for now
    }

    pub fn identifier(&self) -> PolicyID {
        PolicyID {}
    }

    pub fn approves(&self, sigtype: &SignatureType) -> bool {
        false
    }
}

#[derive(PartialEq)]
pub struct PolicyID {}

pub enum Constraint {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
