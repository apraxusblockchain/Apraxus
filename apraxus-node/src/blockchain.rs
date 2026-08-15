use std::collections::HashMap;

use crate::block::Block;
use crate::transaction::SignedTransaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub pending_transactions: Vec<SignedTransaction>,
    pub balances: HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            Vec::new(),
        );

        Blockchain {
            blocks: vec![genesis_block],
            pending_transactions: Vec::new(),
            balances: HashMap::new(),
        }
    }

    pub fn add_genesis_balance(
        &mut self,
        address: String,
        amount: u64,
    ) {
        self.balances.insert(address, amount);
    }

    pub fn add_transaction(
        &mut self,
        transaction: SignedTransaction,
    ) -> bool {
        if !transaction.verify_signature() {
            println!("❌ Invalid signature.");
            return false;
        }

        let balance = self
            .balances
            .get(&transaction.sender)
            .copied()
            .unwrap_or(0);

        if balance < transaction.amount {
            println!("❌ Insufficient balance.");
            return false;
        }

        println!("✅ Transaction added to pool.");

        self.pending_transactions.push(transaction);

        true
    }

    pub fn mine_pending_transactions(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No pending transactions.");
            return;
        }

        let previous_hash =
            self.blocks.last().unwrap().hash.clone();

        let transactions =
            std::mem::take(&mut self.pending_transactions);

        for transaction in &transactions {
            *self
                .balances
                .entry(transaction.sender.clone())
                .or_insert(0) -= transaction.amount;

            *self
                .balances
                .entry(transaction.recipient.clone())
                .or_insert(0) += transaction.amount;
        }

        let block = Block::new(
            self.blocks.len() as u64,
            previous_hash,
            transactions,
        );

        println!(
            "🧱 Block #{} created.",
            block.index
        );

        self.blocks.push(block);
    }

    pub fn balance_of(&self, address: &str) -> u64 {
        self.balances
            .get(address)
            .copied()
            .unwrap_or(0)
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
    pub fn is_chain_valid(&self) -> bool {
    for i in 0..self.blocks.len() {
        let current = &self.blocks[i];

        if !current.is_valid() {
            return false;
        }

        if i == 0 {
            continue;
        }

        let previous = &self.blocks[i - 1];

        if current.previous_hash != previous.hash {
            return false;
        }
    }

    true
}
}
