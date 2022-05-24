extern crate bitcoin;

use crate::error::Error;
use crate::policy::{Policy, PolicyUID};
use crate::signing::{SigRequest, SignedData};
use bitcoin::secp256k1::{self, Secp256k1};
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::ecdsa::EcdsaSig;
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use bitcoin::EcdsaSighashType;
use core::option::Option;
use core::slice::Iter;

/// An "unsealed" HSM cannot perform sensitive operations (e.g. signing). This
/// state is only available for configuration/provisioning.
pub struct UnsealedHsm {
    xpriv: Option<ExtendedPrivKey>,
    policies: Vec<Policy>,
}

/// A "sealed" HSM is available to perform sensitive operations (e.g. signing),
/// but can no longer be modified (e.g. add/remove policies)
pub struct SealedHsm<C: secp256k1::Context> {
    xpriv: ExtendedPrivKey,
    secp: Secp256k1<C>,
    policies: Vec<Policy>,
}

impl UnsealedHsm {
    /// Create a new instance.
    pub fn new() -> UnsealedHsm {
        UnsealedHsm {
            xpriv: None,
            policies: Vec::new(),
        }
    }

    /// Load a seed, and save the derived extended private key, necessary for
    /// secure operations.
    pub fn load_seed(&mut self, seed: &[u8]) -> Result<(), Error> {
        let network = bitcoin::Network::Bitcoin;
        if self.xpriv.is_some() {
            Err(Error::KeySlotFull)
        } else {
            self.xpriv = Some(ExtendedPrivKey::new_master(network, seed)?);
            Ok(())
        }
    }

    /// Delete the saved seed, if any.
    pub fn delete_privkey(&mut self) {
        self.xpriv = None
    }

    /// Add a new policy to the list of saved policies.
    pub fn load_policy(&mut self, new: Policy) -> Result<(), Error> {
        if self
            .policies
            .iter()
            .any(|p| p.identifier() == new.identifier())
        {
            Err(Error::DuplicateEntry)
        } else {
            self.policies.push(new);
            Ok(())
        }
    }

    /// Delete the specified policy, if it exists.
    pub fn delete_policy(&mut self, id: PolicyUID) -> Result<(), Error> {
        if let Some(idx) = self.policies.iter().position(|p| p.identifier() == id) {
            self.policies.remove(idx);
            Ok(())
        } else {
            Err(Error::NotFound)
        }
    }

    pub fn iter_policies(&self) -> Iter<Policy> {
        self.policies.iter()
    }

    /// Helper to test if the HSM is properly configured and able to be sealed,
    /// before attempting the one-way seal() operation.
    pub fn ready_to_seal(&self) -> Result<(), Error> {
        if self.xpriv.is_none() {
            Err(Error::NoPrivKey)
        } else {
            Ok(())
        }
    }

    /// Sealing the HSM will prevent modification of the HSM state, but will
    /// allow secure operations.
    pub fn seal<C: secp256k1::Context>(self, rng_seed: &[u8; 32]) -> Result<SealedHsm<C>, Error> {
        self.ready_to_seal()?;
        let mut secp_ctx = Secp256k1::gen_new();
        secp_ctx.seeded_randomize(rng_seed);
        Ok(SealedHsm {
            xpriv: self.xpriv.unwrap(),
            secp: secp_ctx,
            policies: self.policies,
        })
    }
}

impl<C: secp256k1::Context + secp256k1::Signing> SealedHsm<C> {
    /// Request a signature from the HSM. A signature will only be
    /// performed, if one or more policies approve the transaction.
    pub fn sign(&self, request: SigRequest) -> Result<SignedData, Error> {
        if self.policies.iter().any(|p| p.approves(&request)) {
            match request {
                SigRequest::Psbt(psbt) => self.sign_psbt(psbt),
                SigRequest::Message(msg) => self.sign_message(msg),
            }
        } else {
            Err(Error::Unimplemented)
        }
    }

    fn sign_psbt(&self, mut psbt: Psbt) -> Result<SignedData, Error> {
        // Ensure that every input but finalized one has the `non_witness_utxo`
        if psbt
            .inputs
            .iter()
            .filter(|i| i.final_script_witness.is_none() && i.final_script_sig.is_none())
            .any(|i| i.non_witness_utxo.is_none())
        {
            return Err(Error::MissingNonWitnessUtxo);
        }

        // Refuse to sign the transaction unless every input is using `SIGHASH_ALL`
        if !psbt.inputs.iter().all(|i| {
            if let Some(sighash_type) = i.sighash_type {
                sighash_type.ecdsa_hash_ty() == Ok(EcdsaSighashType::All)
            } else {
                false
            }
        }) {
            return Err(Error::NonStandardSighash);
        }

        // Sign & finalize each input we have a key for
        for (idx, input) in psbt.inputs.iter_mut().enumerate() {
            if let Some(utxo) = &input.non_witness_utxo {
                // Handle non-segwit case
                for (_, (_, path)) in input.bip32_derivation.iter_mut() {
                    let sk = self.xpriv.derive_priv(&self.secp, &path)?;
                    let pk = ExtendedPubKey::from_priv(&self.secp, &sk);
                    input.partial_sigs.insert(
                        pk.to_pub(),
                        EcdsaSig::sighash_all(sk.private_key.sign_ecdsa(
                            secp256k1::Message::from_slice(&utxo.signature_hash(
                                idx,
                                &utxo.input[idx].script_sig,
                                input.sighash_type.unwrap().to_u32(),
                            ))?,
                        )),
                    );
                }
            } else if let Some(_) = input.witness_utxo {
                // Handle segwit case
                return Err(Error::Unimplemented);
            } else {
                return Err(Error::UnsupportedUtxoType);
            }
        }
        Ok(SignedData::Psbt(psbt))
    }

    fn sign_message(&self, _msg: String) -> Result<SignedData, Error> {
        Err(Error::Unimplemented)
    }

    pub fn iter_policies(&self) -> Iter<Policy> {
        self.policies.iter()
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
