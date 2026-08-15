use chrono::Utc;
use sha2::{Digest, Sha256};

struct Block {
    index: u64,
    timestamp: String,
    previous_hash: String,
    data: String,
    hash: String,
}

impl Block {
    fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();

        let input = format!(
            "{}{}{}{}",
            index, timestamp, previous_hash, data
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
            data,
            hash,
        }
    }
}

fn main() {
    let genesis_block = Block::new(
        0,
        "0".to_string(),
        "Apraxus Genesis Block".to_string(),
    );

    println!("=================================");
    println!("        APRAXUS BLOCKCHAIN");
    println!("=================================");
    println!("Block Index:   {}", genesis_block.index);
    println!("Timestamp:     {}", genesis_block.timestamp);
    println!("Previous Hash: {}", genesis_block.previous_hash);
    println!("Data:          {}", genesis_block.data);
    println!("Block Hash:    {}", genesis_block.hash);
    println!("=================================");
}
