use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::transaction::SignedTransaction;

pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub previous_hash: String,
    pub transactions: Vec<SignedTransaction>,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<SignedTransaction>,
    ) -> Self {
        let timestamp = Utc::now().to_rfc3339();

        let transaction_data = transactions
            .iter()
            .map(|tx| tx.message())
            .collect::<String>();

        let input = format!(
            "{}{}{}{}",
            index,
            timestamp,
            previous_hash,
            transaction_data
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());

        let hash = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        Block {
            index,
            timestamp,
            previous_hash,
            transactions,
            hash,
        }
    }
}
