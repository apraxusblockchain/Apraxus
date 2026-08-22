use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

use getrandom::{SysRng, rand_core::UnwrapErr};

use sha2::{Digest, Sha256};

use std::fs;

use crate::blockchain::APXS_DEFAULT_FEE;
use crate::transaction::SignedTransaction;

// =================================
// APXS WALLET
// =================================

pub struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,

    pub address: String,
}

impl Wallet {
    // =================================
    // CREATE NEW WALLET
    // =================================

    pub fn new() -> Self {
        let mut rng = UnwrapErr(SysRng);

        let signing_key = SigningKey::generate(&mut rng);

        Self::from_signing_key(signing_key)
    }

    // =================================
    // CREATE WALLET FROM PRIVATE KEY
    // =================================

    fn from_signing_key(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();

        let address = Self::address_from_public_key(&verifying_key);

        Wallet {
            signing_key,
            verifying_key,
            address,
        }
    }

    // =================================
    // CREATE ADDRESS
    // =================================

    fn address_from_public_key(public_key: &VerifyingKey) -> String {
        let mut hasher = Sha256::new();

        hasher.update(public_key.as_bytes());

        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }

    // =================================
    // SAVE WALLET
    // =================================

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let private_key = self.signing_key.to_bytes();

        fs::write(path, private_key)
            .map_err(|error| format!("Failed to save wallet: {}", error))?;

        Ok(())
    }

    // =================================
    // LOAD WALLET
    // =================================

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path).map_err(|error| format!("Failed to read wallet: {}", error))?;

        // Ed25519 private keys are
        // exactly 32 bytes.

        if bytes.len() != 32 {
            return Err("Invalid wallet file: private key must be 32 bytes.".to_string());
        }

        let mut key_bytes = [0u8; 32];

        key_bytes.copy_from_slice(&bytes);

        let signing_key = SigningKey::from_bytes(&key_bytes);

        Ok(Self::from_signing_key(signing_key))
    }

    // =================================
    // CREATE SIGNED TRANSACTION
    // =================================

    pub fn create_transaction(
        &self,
        recipient: &str,
        amount: u64,
        nonce: u64,
    ) -> SignedTransaction {
        // =================================
        // APXS TRANSACTION FEE
        // =================================

        let fee = APXS_DEFAULT_FEE;

        // =================================
        // TRANSACTION MESSAGE
        // =================================
        //
        // IMPORTANT:
        // This order MUST match transaction.rs:
        //
        // sender + recipient + amount + nonce + fee
        //
        // The fee is included in the signed
        // message so nobody can change it
        // after signing.

        let message = format!("{}{}{}{}{}", self.address, recipient, amount, nonce, fee);

        // =================================
        // SIGN TRANSACTION
        // =================================

        let signature: Signature = self.signing_key.sign(message.as_bytes());

        // =================================
        // CREATE SIGNED TRANSACTION
        // =================================

        SignedTransaction {
            sender: self.address.clone(),

            recipient: recipient.to_string(),

            amount,

            nonce,

            fee,

            public_key: self.verifying_key,

            signature,
        }
    }
}
