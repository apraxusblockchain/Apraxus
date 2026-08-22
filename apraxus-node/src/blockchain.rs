use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::block::Block;
use crate::transaction::SignedTransaction;

// =================================
// APXS TOKEN CONFIGURATION
// =================================

/// Maximum APXS supply.
pub const APXS_MAX_SUPPLY: u64 = 100_000_000_000_000_000;

/// Minimum transaction fee.
pub const APXS_DEFAULT_FEE: u64 = 100_000;

/// Internal address where transaction fees are collected.
pub const APXS_FEE_POOL_ADDRESS: &str = "apxs_fee_pool";

// =================================
// BLOCKCHAIN
// =================================

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,

    pub pending_transactions: Vec<SignedTransaction>,

    pub balances: HashMap<String, u64>,

    #[serde(default)]
    pub total_supply: u64,
}

impl Blockchain {
    // =================================
    // TOKEN INFORMATION
    // =================================

    pub fn token_name() -> &'static str {
        "Apraxus"
    }

    pub fn token_symbol() -> &'static str {
        "APXS"
    }

    pub fn decimals() -> u8 {
        8
    }

    pub fn max_supply() -> u64 {
        APXS_MAX_SUPPLY
    }

    pub fn atomic_units_per_apxs() -> u64 {
        100_000_000
    }

    pub fn default_fee() -> u64 {
        APXS_DEFAULT_FEE
    }

    // =================================
    // CREATE NEW BLOCKCHAIN
    // =================================

    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), Vec::new());

        Blockchain {
            blocks: vec![genesis_block],
            pending_transactions: Vec::new(),
            balances: HashMap::new(),
            total_supply: 0,
        }
    }

    // =================================
    // GENESIS BALANCE
    // =================================

    pub fn add_genesis_balance(&mut self, address: String, amount: u64) {
        if self.balances.contains_key(&address) {
            println!("❌ Genesis balance already exists for this address.");

            return;
        }

        let new_total = match self.total_supply.checked_add(amount) {
            Some(value) => value,

            None => {
                println!("❌ APXS supply calculation overflow.");

                return;
            }
        };

        if new_total > APXS_MAX_SUPPLY {
            println!("❌ APXS maximum supply exceeded.");

            return;
        }

        self.balances.insert(address, amount);

        self.total_supply = new_total;

        println!("🪙 Created {} APXS.", amount);

        println!(
            "📊 Total APXS supply: {} / {}",
            self.total_supply, APXS_MAX_SUPPLY
        );
    }

    // =================================
    // ADD TRANSACTION
    // =================================

    pub fn add_transaction(&mut self, transaction: SignedTransaction) -> bool {
        if !transaction.verify_signature() {
            println!("❌ Invalid signature.");

            return false;
        }

        if transaction.fee < APXS_DEFAULT_FEE {
            println!("❌ Transaction fee is below the APXS minimum fee.");

            return false;
        }

        // =================================
        // PENDING DUPLICATE
        // =================================

        let pending_duplicate = self
            .pending_transactions
            .iter()
            .any(|existing| Self::same_transaction(existing, &transaction));

        if pending_duplicate {
            println!("❌ Duplicate transaction.");

            return false;
        }

        // =================================
        // CONFIRMED DUPLICATE
        // =================================

        let confirmed_duplicate = self.blocks.iter().any(|block| {
            block
                .transactions
                .iter()
                .any(|existing| Self::same_transaction(existing, &transaction))
        });

        if confirmed_duplicate {
            println!("❌ Transaction already confirmed.");

            return false;
        }

        // =================================
        // SENDER BALANCE
        // =================================

        let confirmed_balance = self.balance_of(&transaction.sender);

        // =================================
        // PENDING OUTGOING
        // =================================

        let pending_outgoing = self
            .pending_transactions
            .iter()
            .filter(|pending| pending.sender == transaction.sender)
            .fold(0u64, |total, pending| {
                total
                    .saturating_add(pending.amount)
                    .saturating_add(pending.fee)
            });

        // =================================
        // TOTAL COST
        // =================================

        let required = match transaction.amount.checked_add(transaction.fee) {
            Some(value) => value,

            None => {
                println!("❌ Transaction amount + fee overflow.");

                return false;
            }
        };

        let available_balance = confirmed_balance.saturating_sub(pending_outgoing);

        if available_balance < required {
            println!("❌ Insufficient balance.");

            println!("Required: {} APXS", required);

            println!("Available: {} APXS", available_balance);

            return false;
        }

        // =================================
        // ADD TO MEMPOOL
        // =================================

        println!("✅ Transaction added to pool.");

        println!("💸 Amount: {} APXS", transaction.amount);

        println!("⛽ Fee: {} APXS", transaction.fee);

        println!("💰 Total cost: {} APXS", required);

        self.pending_transactions.push(transaction);

        true
    }

    // =================================
    // TRANSACTION COMPARISON
    // =================================

    fn same_transaction(a: &SignedTransaction, b: &SignedTransaction) -> bool {
        a.sender == b.sender
            && a.recipient == b.recipient
            && a.amount == b.amount
            && a.fee == b.fee
            && a.nonce == b.nonce
    }

    // =================================
    // MINE PENDING TRANSACTIONS
    // =================================

    pub fn mine_pending_transactions(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No pending transactions.");

            return;
        }

        let previous_hash = self.blocks.last().unwrap().hash.clone();

        let transactions = std::mem::take(&mut self.pending_transactions);

        let mut valid_transactions = Vec::new();

        // =================================
        // EXECUTE TRANSACTIONS
        // =================================

        for transaction in transactions {
            if !transaction.verify_signature() {
                println!("❌ Transaction skipped: invalid signature.");

                continue;
            }

            if transaction.fee < APXS_DEFAULT_FEE {
                println!("❌ Transaction skipped: fee below minimum.");

                continue;
            }

            let total_cost = match transaction.amount.checked_add(transaction.fee) {
                Some(value) => value,

                None => {
                    println!("❌ Transaction skipped: amount + fee overflow.");

                    continue;
                }
            };

            let sender_balance = self.balance_of(&transaction.sender);

            if sender_balance < total_cost {
                println!("❌ Transaction skipped: insufficient balance.");

                continue;
            }

            // =================================
            // SUBTRACT FROM SENDER
            // =================================

            {
                let sender = self.balances.entry(transaction.sender.clone()).or_insert(0);

                *sender -= total_cost;
            }

            // =================================
            // ADD TO RECIPIENT
            // =================================

            {
                let recipient = self
                    .balances
                    .entry(transaction.recipient.clone())
                    .or_insert(0);

                *recipient = recipient.saturating_add(transaction.amount);
            }

            // =================================
            // COLLECT FEE
            // =================================

            {
                let fee_pool = self
                    .balances
                    .entry(APXS_FEE_POOL_ADDRESS.to_string())
                    .or_insert(0);

                *fee_pool = fee_pool.saturating_add(transaction.fee);
            }

            println!("💸 Transfer: {} APXS", transaction.amount);

            println!("⛽ Fee collected: {} APXS", transaction.fee);

            valid_transactions.push(transaction);
        }

        // =================================
        // NO VALID TRANSACTIONS
        // =================================

        if valid_transactions.is_empty() {
            println!("⚠️ No valid transactions available for mining.");

            return;
        }

        // =================================
        // CREATE BLOCK
        // =================================

        let block = Block::new(self.blocks.len() as u64, previous_hash, valid_transactions);

        println!("🧱 Block #{} created.", block.index);

        self.blocks.push(block);
    }

    // =================================
    // BALANCE
    // =================================

    pub fn balance_of(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    // =================================
    // FEE POOL
    // =================================

    pub fn fee_pool_balance(&self) -> u64 {
        self.balance_of(APXS_FEE_POOL_ADDRESS)
    }

    // =================================
    // BLOCK COUNT
    // =================================

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    // =================================
    // CALCULATED SUPPLY
    // =================================

    fn calculated_supply(&self) -> u64 {
        self.balances.values().fold(0u64, |total, balance| {
            total.checked_add(*balance).unwrap_or(u64::MAX)
        })
    }

    // =================================
    // CHAIN VALIDATION
    // =================================

    pub fn is_chain_valid(&self) -> bool {
        // =================================
        // BASIC CHECK
        // =================================

        if self.blocks.is_empty() {
            return false;
        }

        // =================================
        // MAX SUPPLY
        // =================================

        if self.total_supply > APXS_MAX_SUPPLY {
            return false;
        }

        // =================================
        // SUPPLY
        // =================================

        if self.calculated_supply() != self.total_supply {
            return false;
        }

        // =================================
        // VERIFY BLOCKS
        // =================================

        for i in 0..self.blocks.len() {
            let current = &self.blocks[i];

            // =================================
            // BLOCK HASH
            // =================================

            if !current.is_valid() {
                return false;
            }

            // =================================
            // GENESIS
            // =================================

            if i == 0 {
                if current.index != 0 {
                    return false;
                }

                if current.previous_hash != "0" {
                    return false;
                }

                if !current.transactions.is_empty() {
                    return false;
                }

                continue;
            }

            // =================================
            // PREVIOUS BLOCK
            // =================================

            let previous = &self.blocks[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            if current.index != previous.index + 1 {
                return false;
            }

            // =================================
            // TRANSACTIONS
            // =================================

            for transaction in &current.transactions {
                if !transaction.verify_signature() {
                    return false;
                }

                // Legacy fee = 0 is allowed
                // only for old transactions.
                //
                // New transactions must use
                // APXS_DEFAULT_FEE or higher.

                if transaction.fee != 0 && transaction.fee < APXS_DEFAULT_FEE {
                    return false;
                }
            }
        }

        true
    }

    // =================================
    // PEER CHAIN COMPARISON
    // =================================

    pub fn is_longer_than(&self, other: &Blockchain) -> bool {
        self.blocks.len() > other.blocks.len()
    }

    // =================================
    // REPLACE IF LONGER
    // =================================

    pub fn replace_if_longer(&mut self, incoming: Blockchain) -> bool {
        println!("🔄 Checking incoming blockchain...");

        if !incoming.is_chain_valid() {
            println!("❌ Incoming blockchain is invalid.");

            return false;
        }

        println!("🔐 Incoming blockchain valid.");

        println!("📊 Local blocks: {}", self.blocks.len());

        println!("📊 Incoming blocks: {}", incoming.blocks.len());

        if incoming.blocks.len() <= self.blocks.len() {
            println!("ℹ️ Local blockchain is already equal or longer.");

            return false;
        }

        println!(
            "🔄 Replacing local blockchain: {} blocks -> {} blocks",
            self.blocks.len(),
            incoming.blocks.len()
        );

        self.blocks = incoming.blocks;

        self.pending_transactions = incoming.pending_transactions;

        self.balances = incoming.balances;

        self.total_supply = incoming.total_supply;

        println!("✅ Blockchain synchronized successfully.");

        true
    }

    // =================================
    // SYNCHRONIZE WITH PEER
    // =================================

    pub fn replace_with_peer(&mut self, peer_blockchain: Blockchain) -> bool {
        println!("🔄 Checking peer blockchain for synchronization...");

        if !peer_blockchain.is_chain_valid() {
            println!("❌ Peer blockchain is invalid.");

            return false;
        }

        println!("🔐 Peer blockchain valid.");

        if peer_blockchain.blocks.len() <= self.blocks.len() {
            println!("ℹ️ Local blockchain is already equal or longer.");

            return false;
        }

        println!("📥 Peer blockchain is longer.");

        println!("📊 Local blocks: {}", self.blocks.len());

        println!("📊 Peer blocks: {}", peer_blockchain.blocks.len());

        self.blocks = peer_blockchain.blocks;

        self.pending_transactions = peer_blockchain.pending_transactions;

        self.balances = peer_blockchain.balances;

        self.total_supply = peer_blockchain.total_supply;

        println!("✅ Blockchain synchronized successfully.");

        true
    }

    // =================================
    // SAVE BLOCKCHAIN
    // =================================

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self).map_err(|error| error.to_string())?;

        fs::write(path, json).map_err(|error| error.to_string())?;

        Ok(())
    }

    // =================================
    // LOAD BLOCKCHAIN
    // =================================

    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let json = fs::read_to_string(path).map_err(|error| error.to_string())?;

        let mut blockchain: Blockchain =
            serde_json::from_str(&json).map_err(|error| error.to_string())?;

        // =================================
        // LEGACY SUPPLY RECOVERY
        // =================================

        if blockchain.total_supply == 0 && !blockchain.balances.is_empty() {
            let calculated = blockchain.calculated_supply();

            if calculated <= APXS_MAX_SUPPLY {
                blockchain.total_supply = calculated;
            }
        }

        // =================================
        // DETAILED VALIDATION DEBUG
        // =================================

        println!("🔎 Blockchain validation:");

        println!("   Blocks: {}", blockchain.blocks.len());

        println!("   Recorded supply: {}", blockchain.total_supply);

        println!("   Calculated supply: {}", blockchain.calculated_supply());

        println!("   Max supply: {}", APXS_MAX_SUPPLY);

        // =================================
        // CHECK SUPPLY
        // =================================

        if blockchain.total_supply > APXS_MAX_SUPPLY {
            println!("❌ Validation failed: maximum supply exceeded.");

            return Err("Blockchain validation failed: maximum supply exceeded.".to_string());
        }

        if blockchain.calculated_supply() != blockchain.total_supply {
            println!("❌ Validation failed: supply mismatch.");

            return Err("Blockchain validation failed: supply mismatch.".to_string());
        }

        // =================================
        // CHECK EVERY BLOCK
        // =================================

        for i in 0..blockchain.blocks.len() {
            let block = &blockchain.blocks[i];

            if !block.is_valid() {
                println!(
                    "❌ Validation failed: block #{} hash is invalid.",
                    block.index
                );

                return Err(format!(
                    "Blockchain validation failed: block #{} hash is invalid.",
                    block.index
                ));
            }

            if i == 0 {
                if block.index != 0 {
                    println!("❌ Validation failed: genesis index is invalid.");

                    return Err("Invalid genesis block index.".to_string());
                }

                if block.previous_hash != "0" {
                    println!("❌ Validation failed: genesis previous hash is invalid.");

                    return Err("Invalid genesis previous hash.".to_string());
                }

                continue;
            }

            let previous = &blockchain.blocks[i - 1];

            if block.previous_hash != previous.hash {
                println!(
                    "❌ Validation failed: block #{} previous hash does not match block #{}.",
                    block.index, previous.index
                );

                return Err(format!(
                    "Blockchain validation failed: block #{} previous hash mismatch.",
                    block.index
                ));
            }

            if block.index != previous.index + 1 {
                println!(
                    "❌ Validation failed: block numbering error at block #{}.",
                    block.index
                );

                return Err(format!(
                    "Blockchain validation failed: block numbering error at block #{}.",
                    block.index
                ));
            }

            for transaction in &block.transactions {
                if !transaction.verify_signature() {
                    println!(
                        "❌ Validation failed: invalid transaction signature in block #{}.",
                        block.index
                    );

                    return Err(format!(
                        "Blockchain validation failed: invalid transaction in block #{}.",
                        block.index
                    ));
                }
            }
        }

        println!("✅ Blockchain validation successful.");

        Ok(blockchain)
    }
}
