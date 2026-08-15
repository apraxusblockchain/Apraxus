use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

use crate::transaction::SignedTransaction;

#[derive(Serialize, Deserialize)]
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

        let hash = Self::calculate_hash(
            index,
            &timestamp,
            &previous_hash,
            &transactions,
        );

        Block {
            index,
            timestamp,
            previous_hash,
            transactions,
            hash,
        }
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: &str,
        previous_hash: &str,
        transactions: &[SignedTransaction],
    ) -> String {
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

        hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }

    pub fn is_valid(&self) -> bool {
        let calculated_hash = Self::calculate_hash(
            self.index,
            &self.timestamp,
            &self.previous_hash,
            &self.transactions,
        );

        self.hash == calculated_hash
    }
}
