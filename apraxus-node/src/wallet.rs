use ed25519_dalek::{
    Signature, Signer, SigningKey, VerifyingKey,
};
use getrandom::{rand_core::UnwrapErr, SysRng};
use sha2::{Digest, Sha256};

use crate::transaction::SignedTransaction;

pub struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = UnwrapErr(SysRng);

        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let mut hasher = Sha256::new();
        hasher.update(verifying_key.as_bytes());

        let address = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        Wallet {
            signing_key,
            verifying_key,
            address,
        }
    }

    pub fn create_transaction(
        &self,
        recipient: &str,
        amount: u64,
        nonce: u64,
    ) -> SignedTransaction {
        let message = format!(
            "{}{}{}{}",
            self.address,
            recipient,
            amount,
            nonce
        );

        let signature: Signature =
            self.signing_key.sign(message.as_bytes());

        SignedTransaction {
            sender: self.address.clone(),
            recipient: recipient.to_string(),
            amount,
            nonce,
            public_key: self.verifying_key,
            signature,
        }
    }
}
