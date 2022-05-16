extern crate bitcoin;

use crate::error::Error;
use crate::policy::{Policy, PolicyUID};
use crate::signing::SigRequest;
use bitcoin::util::bip32::ExtendedPrivKey;
use core::option::Option;
use core::slice::Iter;

/// HSM context struct
struct Hsm {
    xpriv: Option<ExtendedPrivKey>,
    policies: Vec<Policy>,
}

/// An "unsealed" HSM cannot perform sensitive operations (e.g. signing). This
/// state is only available for configuration/provisioning.
pub struct Unsealed(Hsm);

/// A "sealed" HSM is available to perform sensitive operations (e.g. signing),
/// but can no longer be modified (e.g. add/remove policies)
pub struct Sealed(Hsm);

impl Hsm {
    /// Return an iterator over the saved policies.
    fn iter_policies(&self) -> Iter<Policy> {
        self.policies.iter()
    }
}

impl Unsealed {
    /// Create a new instance.
    pub fn new() -> Unsealed {
        Unsealed(Hsm {
            xpriv: None,
            policies: Vec::new(),
        })
    }

    /// Load a seed, and save the derived extended private key, necessary for
    /// secure operations.
    pub fn load_seed(&mut self, seed: &[u8]) -> Result<(), Error> {
        let network = bitcoin::Network::Bitcoin;
        if self.0.xpriv.is_some() {
            Err(Error::KeySlotFull)
        } else {
            self.0.xpriv = Some(ExtendedPrivKey::new_master(network, seed)?);
            Ok(())
        }
    }

    /// Delete the saved seed, if any.
    pub fn delete_privkey(&mut self) {
        self.0.xpriv = None
    }

    /// Add a new policy to the list of saved policies.
    pub fn load_policy(&mut self, new: Policy) -> Result<(), Error> {
        if self
            .0
            .policies
            .iter()
            .any(|p| p.identifier() == new.identifier())
        {
            Err(Error::DuplicateEntry)
        } else {
            self.0.policies.push(new);
            Ok(())
        }
    }

    /// Delete the specified policy, if it exists.
    pub fn delete_policy(&mut self, id: PolicyUID) -> Result<(), Error> {
        if let Some(idx) = self.0.policies.iter().position(|p| p.identifier() == id) {
            self.0.policies.remove(idx);
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn iter_policies(&self) -> Iter<Policy> {
        self.0.iter_policies()
    }

    /// Helper to test if the HSM is properly configured and able to be sealed,
    /// before attempting the one-way seal() operation.
    pub fn ready_to_seal(&self) -> Result<(), Error> {
        if self.0.xpriv.is_none() {
            Err(Error::NoPrivKey)
        } else {
            Ok(())
        }
    }

    /// Sealing the HSM will prevent modification of the HSM state, but will
    /// allow secure operations.
    pub fn seal(self) -> Result<Sealed, Error> {
        self.ready_to_seal()?;
        Ok(Sealed(self.0))
    }
}

impl Sealed {
    /// Request a signature from the HSM. A signature will only be
    /// performed, if one or more policies approve the transaction.
    pub fn sign(&self, request: SigRequest) -> Result<(), Error> {
        if self.0.policies.iter().any(|p| p.approves(&request)) {
            Ok(())
        } else {
            Err(Error::Unimplemented)
        }
    }

    pub fn iter_policies(&self) -> Iter<Policy> {
        self.0.iter_policies()
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
