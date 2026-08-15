use ed25519_dalek::{Signature, Verifier, VerifyingKey};

#[derive(Clone)]
pub struct SignedTransaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub nonce: u64,
    pub public_key: VerifyingKey,
    pub signature: Signature,
}

impl SignedTransaction {
    pub fn message(&self) -> String {
        format!(
            "{}{}{}{}",
            self.sender,
            self.recipient,
            self.amount,
            self.nonce
        )
    }

    pub fn verify_signature(&self) -> bool {
        let message = self.message();

        self.public_key
            .verify(message.as_bytes(), &self.signature)
            .is_ok()
    }
}
