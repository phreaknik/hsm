extern crate bitcoin;

use crate::error::Error;
use crate::policy::{Policy, PolicyID};
use core::slice::Iter;

/// HSM context struct. Holds data necessary to evaluate and sign PSBTs
pub struct Hsm {
    seed: String,
    policies: Vec<Policy>,
}

impl Hsm {
    /// Create a new PsbtHsm instance.
    pub fn new() -> Hsm {
        Hsm {
            seed: String::new(),
            policies: Vec::new(),
        }
    }

    pub fn has_seed(&self) -> bool {
        false
    }

    /// Save a seed with which to sign transactions.
    pub fn add_seed(&self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Delete the saved seed, if any.
    pub fn del_seed(&self) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Add a new policy to the list of saved policies.
    pub fn add_policy(&self, p: Policy) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Delete the specified policy, if it exists.
    pub fn del_policy(&self, p: PolicyID) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    /// Return an iterator over the saved policies.
    pub fn iter_policies(&self) -> Iter<Policy> {
        self.policies.iter()
    }

    /// Request a signature for the provided PSBT. A signature will only be
    /// performed, if one or more policies approve the transaction.
    pub fn sign(&self) -> Result<(), Error> {
        Err(Error::Unimplemented)
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
