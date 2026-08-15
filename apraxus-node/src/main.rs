use ed25519_dalek::{
    Signature, Signer, SigningKey, Verifier, VerifyingKey,
};
use getrandom::{rand_core::UnwrapErr, SysRng};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl Wallet {
    fn new() -> Self {
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

    fn sign_transaction(
        &self,
        recipient: &str,
        amount: u64,
        nonce: u64,
    ) -> SignedTransaction {
        let message = format!(
            "{}{}{}",
            self.address, recipient, amount
        );

        let signature = self.signing_key.sign(message.as_bytes());

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

#[derive(Clone)]
struct SignedTransaction {
    sender: String,
    recipient: String,
    amount: u64,
    nonce: u64,
    public_key: VerifyingKey,
    signature: Signature,
}

impl SignedTransaction {
    fn message(&self) -> String {
        format!(
            "{}{}{}",
            self.sender,
            self.recipient,
            self.amount
        )
    }

    fn verify_signature(&self) -> bool {
        let message = self.message();

        self.public_key
            .verify(message.as_bytes(), &self.signature)
            .is_ok()
    }
}

struct Blockchain {
    balances: HashMap<String, u64>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            balances: HashMap::new(),
        }
    }

    fn add_genesis_balance(
        &mut self,
        address: String,
        amount: u64,
    ) {
        self.balances.insert(address, amount);
    }

    fn process_transaction(
        &mut self,
        transaction: &SignedTransaction,
    ) -> bool {
        if !transaction.verify_signature() {
            println!("❌ Transaction rejected: invalid signature.");
            return false;
        }

        let sender_balance = *self
            .balances
            .get(&transaction.sender)
            .unwrap_or(&0);

        if sender_balance < transaction.amount {
            println!("❌ Transaction rejected: insufficient balance.");
            return false;
        }

        *self
            .balances
            .entry(transaction.sender.clone())
            .or_insert(0) -= transaction.amount;

        *self
            .balances
            .entry(transaction.recipient.clone())
            .or_insert(0) += transaction.amount;

        println!("✅ Transaction accepted.");
        true
    }

    fn balance_of(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}

fn main() {
    println!("=================================");
    println!("     APRAXUS SIGNED TRANSACTION");
    println!("=================================");

    let alice = Wallet::new();
    let bob = Wallet::new();

    let mut blockchain = Blockchain::new();

    // Development-only genesis allocation.
    blockchain.add_genesis_balance(
        alice.address.clone(),
        100,
    );

    println!("\nAlice address:");
    println!("{}", alice.address);

    println!("\nBob address:");
    println!("{}", bob.address);

    println!("\nInitial balances:");
    println!(
        "Alice: {} APXV",
        blockchain.balance_of(&alice.address)
    );
    println!(
        "Bob:   {} APXV",
        blockchain.balance_of(&bob.address)
    );

    println!("\nCreating signed transaction...");
    println!("Alice -> Bob : 25 APXV");

    let transaction =
        alice.sign_transaction(&bob.address, 25, 0);

    println!(
        "Signature valid: {}",
        transaction.verify_signature()
    );

    println!("\nSubmitting transaction to Apraxus...");

    blockchain.process_transaction(&transaction);

    println!("\nFinal balances:");
    println!(
        "Alice: {} APXV",
        blockchain.balance_of(&alice.address)
    );
    println!(
        "Bob:   {} APXV",
        blockchain.balance_of(&bob.address)
    );

    println!("=================================");
}
