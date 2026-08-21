use ed25519_dalek::{
    Signature,
    Verifier,
    VerifyingKey,
};

use serde::{
    Deserialize,
    Serialize,
};

use sha2::{
    Digest,
    Sha256,
};

// =================================
// APXS TRANSACTION
// =================================

#[derive(
    Clone,
    Serialize,
    Deserialize,
)]
pub struct SignedTransaction {
    // =================================
    // TRANSACTION DATA
    // =================================

    pub sender: String,

    pub recipient: String,

    pub amount: u64,

    pub nonce: u64,

    // =================================
    // TRANSACTION FEE
    // =================================
    //
    // New transactions contain a fee.
    //
    // Older blockchain files were created
    // before fees existed, so their JSON
    // transactions may not contain this field.
    //
    // Missing fee = 0 means LEGACY transaction.
    //
    // New Wallet transactions use
    // APXS_DEFAULT_FEE.

    #[serde(default)]
    pub fee: u64,

    // =================================
    // CRYPTOGRAPHIC DATA
    // =================================

    pub public_key: VerifyingKey,

    pub signature: Signature,
}

// =================================
// LEGACY TRANSACTION MESSAGE
// =================================
//
// Old APXS transactions were signed using:
//
// sender + recipient + amount + nonce
//
// No fee existed at that time.

fn legacy_message(
    sender: &str,
    recipient: &str,
    amount: u64,
    nonce: u64,
) -> String {
    format!(
        "{}{}{}{}",
        sender,
        recipient,
        amount,
        nonce,
    )
}

impl SignedTransaction {

    // =================================
    // TRANSACTION MESSAGE
    // =================================
    //
    // IMPORTANT:
    //
    // This order MUST exactly match wallet.rs.
    //
    // wallet.rs signs:
    //
    // sender
    // + recipient
    // + amount
    // + fee
    // + nonce
    //
    // Therefore verification uses
    // the exact same order.

    pub fn message(&self) -> String {

    format!(
    "{}{}{}{}{}",
    self.sender,
    self.recipient,
    self.amount,
    self.nonce,
    self.fee,
)
}

    // =================================
    // DERIVE ADDRESS FROM PUBLIC KEY
    // =================================
    //
    // APXS address =
    // SHA-256(public key)

    fn derived_address(&self) -> String {
        let mut hasher =
            Sha256::new();

        hasher.update(
            self.public_key.as_bytes()
        );

        hasher
            .finalize()
            .iter()
            .map(|byte| {
                format!("{:02x}", byte)
            })
            .collect::<String>()
    }

    // =================================
    // VERIFY TRANSACTION
    // =================================

    pub fn verify_signature(&self) -> bool {

        // =================================
        // BASIC VALIDATION
        // =================================

        // Zero-value transactions
        // are not valid transfers.

        if self.amount == 0 {
            return false;
        }

        // Sender must exist.

        if self.sender.is_empty() {
            return false;
        }

        // Recipient must exist.

        if self.recipient.is_empty() {
            return false;
        }

        // Sender cannot send to itself.

        if self.sender == self.recipient {
            return false;
        }

        // =================================
        // VERIFY SENDER OWNERSHIP
        // =================================
        //
        // The sender address must be the
        // SHA-256 hash of the supplied
        // public key.

        let derived_address =
            self.derived_address();

        if derived_address
            != self.sender
        {
            return false;
        }

        // =================================
        // LEGACY TRANSACTION
        // =================================
        //
        // fee == 0 means this transaction
        // came from the old blockchain format.
        //
        // Old transactions were signed WITHOUT
        // a fee, so verify the old message format.

        if self.fee == 0 {

            let message =
                legacy_message(
                    &self.sender,
                    &self.recipient,
                    self.amount,
                    self.nonce,
                );

            return self
                .public_key
                .verify(
                    message.as_bytes(),
                    &self.signature,
                )
                .is_ok();
        }

        // =================================
        // NEW TRANSACTION
        // =================================
        //
        // New transactions include the fee
        // inside the signed message.
        //
        // This protects the fee from being
        // modified after signing.

        let message =
            self.message();

        self.public_key
            .verify(
                message.as_bytes(),
                &self.signature,
            )
            .is_ok()
    }
}