use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

use crate::blockchain::Blockchain;
use crate::transaction::SignedTransaction;

pub struct Network {
    pub address: String,
    pub blockchain_file: String,
}

impl Network {
    pub fn new(address: &str, blockchain_file: &str) -> Self {
        Network {
            address: address.to_string(),
            blockchain_file: blockchain_file.to_string(),
        }
    }

    // =================================
    // START NODE
    // =================================

    pub fn start(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        println!("🌐 Apraxus node listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("🔗 Peer connected: {:?}", stream.peer_addr());

                    Self::handle_connection(stream, &self.blockchain_file);
                }

                Err(error) => {
                    println!("❌ Connection failed: {}", error);
                }
            }
        }

        Ok(())
    }

    // =================================
    // CONNECT / SYNCHRONIZE WITH PEER
    // =================================

    pub fn connect_to_peer(&self, peer_address: &str) -> io::Result<()> {
        println!("\n🔗 Connecting to peer {}...", peer_address);

        let mut stream = TcpStream::connect(peer_address)?;

        println!("✅ Connected to peer {}", peer_address);

        // =================================
        // RECEIVE HANDSHAKE
        // =================================

        let mut handshake_buffer = [0u8; 1024];

        let bytes_read = stream.read(&mut handshake_buffer)?;

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&handshake_buffer[..bytes_read]);

            println!("📨 Peer response: {}", response.trim());
        }

        // =================================
        // REQUEST BLOCKCHAIN
        // =================================

        stream.write_all(b"GET_BLOCKCHAIN\n")?;

        println!("📤 Blockchain request sent.");

        // Tell peer that request is complete.
        stream.shutdown(Shutdown::Write)?;

        println!("📥 Waiting for blockchain response...");

        // =================================
        // RECEIVE BLOCKCHAIN
        // =================================

        let mut blockchain_data = Vec::new();

        stream.read_to_end(&mut blockchain_data)?;

        if blockchain_data.is_empty() {
            println!("⚠️ Peer returned no blockchain.");

            return Ok(());
        }

        // =================================
        // DECODE BLOCKCHAIN
        // =================================

        let peer_blockchain = match serde_json::from_slice::<Blockchain>(&blockchain_data) {
            Ok(blockchain) => blockchain,

            Err(error) => {
                println!("❌ Failed to decode peer blockchain: {}", error);

                return Ok(());
            }
        };

        println!("📦 Blockchain received.");

        println!("📊 Peer blocks: {}", peer_blockchain.block_count());

        println!("💰 Peer total supply: {}", peer_blockchain.total_supply);

        // =================================
        // VALIDATE PEER CHAIN
        // =================================

        if !peer_blockchain.is_chain_valid() {
            println!("❌ Peer blockchain is invalid.");

            return Ok(());
        }

        println!("🔐 Peer chain valid: true");

        // =================================
        // CHECK LOCAL BLOCKCHAIN
        // =================================

        let local_exists = std::path::Path::new(&self.blockchain_file).exists();

        // =================================
        // NO LOCAL BLOCKCHAIN
        // =================================

        if !local_exists {
            println!("🆕 No local blockchain found.");

            println!("📥 Adopting peer blockchain...");

            match peer_blockchain.save_to_file(&self.blockchain_file) {
                Ok(()) => {
                    println!("💾 Peer blockchain saved locally.");

                    println!("📊 Local blocks: {}", peer_blockchain.block_count());

                    println!(
                        "💰 Total APXS supply: {} / 1000000000",
                        peer_blockchain.total_supply
                    );

                    println!("✅ Initial blockchain synchronization complete.");
                }

                Err(error) => {
                    println!("❌ Failed to save peer blockchain: {}", error);
                }
            }

            return Ok(());
        }

        // =================================
        // LOAD LOCAL BLOCKCHAIN
        // =================================

        let mut local_blockchain = match Blockchain::load_from_file(&self.blockchain_file) {
            Ok(blockchain) => blockchain,

            Err(error) => {
                println!("❌ Failed to load local blockchain: {}", error);

                return Ok(());
            }
        };

        println!("📂 Local blocks: {}", local_blockchain.block_count());

        println!("💰 Local total supply: {}", local_blockchain.total_supply);

        // =================================
        // SYNCHRONIZATION
        // =================================

        let replaced = local_blockchain.replace_if_longer(peer_blockchain);

        if !replaced {
            println!("ℹ️ Local blockchain is already equal or longer.");

            return Ok(());
        }

        // =================================
        // SAVE SYNCHRONIZED CHAIN
        // =================================

        match local_blockchain.save_to_file(&self.blockchain_file) {
            Ok(()) => {
                println!("💾 Synchronized blockchain saved.");

                println!("📊 New local blocks: {}", local_blockchain.block_count());

                println!(
                    "💰 New total APXS supply: {}",
                    local_blockchain.total_supply
                );

                println!("✅ Blockchain synchronization successful.");
            }

            Err(error) => {
                println!("❌ Failed to save synchronized blockchain: {}", error);
            }
        }

        Ok(())
    }

    // =================================
    // BROADCAST TRANSACTION
    // =================================

    pub fn broadcast_transaction(
        &self,
        peer_address: &str,
        transaction: &SignedTransaction,
    ) -> io::Result<()> {
        println!("📡 Broadcasting transaction to {}...", peer_address);

        // Verify before sending.
        if !transaction.verify_signature() {
            println!("❌ Transaction has invalid signature.");

            return Ok(());
        }

        println!("🔐 Transaction signature verified.");

        let mut stream = TcpStream::connect(peer_address)?;

        println!("✅ Connected to peer {}", peer_address);

        // =================================
        // RECEIVE HANDSHAKE
        // =================================

        let mut buffer = [0u8; 1024];

        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read > 0 {
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);

            println!("📨 Peer response: {}", response.trim());
        }

        // =================================
        // SERIALIZE TRANSACTION
        // =================================

        let transaction_json = serde_json::to_vec(transaction)
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error.to_string()))?;

        // =================================
        // SEND TRANSACTION
        // =================================

        stream.write_all(b"TRANSACTION\n")?;

        stream.write_all(&transaction_json)?;

        stream.shutdown(Shutdown::Write)?;

        println!("📤 Transaction sent successfully.");

        Ok(())
    }

    // =================================
    // HANDLE INCOMING CONNECTION
    // =================================

    fn handle_connection(mut stream: TcpStream, blockchain_file: &str) {
        // =================================
        // HANDSHAKE
        // =================================

        if let Err(error) = stream.write_all(b"APRAXUS_NODE_READY\n") {
            println!("❌ Failed to send handshake: {}", error);

            return;
        }

        println!("🤝 Handshake sent successfully.");

        // =================================
        // RECEIVE DATA
        // =================================

        let mut data = Vec::new();

        match stream.read_to_end(&mut data) {
            Ok(bytes_read) if bytes_read > 0 => {
                let message = String::from_utf8_lossy(&data);

                // =================================
                // BLOCKCHAIN REQUEST
                // =================================

                if message.trim() == "GET_BLOCKCHAIN" {
                    println!("📨 Received from peer: GET_BLOCKCHAIN");

                    Self::send_blockchain(&mut stream, blockchain_file);

                    return;
                }

                // =================================
                // TRANSACTION
                // =================================

                if let Some(json) = message.strip_prefix("TRANSACTION\n") {
                    println!("📨 Transaction received from peer.");

                    let transaction = match serde_json::from_str::<SignedTransaction>(json) {
                        Ok(transaction) => transaction,

                        Err(error) => {
                            println!("❌ Invalid transaction data: {}", error);

                            return;
                        }
                    };

                    println!("💸 Transaction decoded successfully.");

                    // Verify signature.
                    if !transaction.verify_signature() {
                        println!("❌ Transaction rejected: invalid signature.");

                        return;
                    }

                    println!("🔐 Transaction signature valid.");

                    // Load blockchain.
                    let mut blockchain = match Blockchain::load_from_file(blockchain_file) {
                        Ok(blockchain) => blockchain,

                        Err(error) => {
                            println!("❌ Failed to load blockchain: {}", error);

                            return;
                        }
                    };

                    // Add transaction.
                    let accepted = blockchain.add_transaction(transaction);

                    if !accepted {
                        println!("❌ Transaction rejected by blockchain.");

                        return;
                    }

                    println!("✅ Transaction accepted into pool.");

                    // Save.
                    match blockchain.save_to_file(blockchain_file) {
                        Ok(()) => {
                            println!("💾 Transaction pool saved successfully.");
                        }

                        Err(error) => {
                            println!("❌ Failed to save blockchain: {}", error);
                        }
                    }

                    return;
                }

                // =================================
                // P2P TEST
                // =================================

                if message.trim() == "APRAXUS_P2P_TEST" {
                    if let Err(error) = stream.write_all(b"APRAXUS_P2P_TEST_ACK\n") {
                        println!("❌ Failed to send response: {}", error);
                    } else {
                        println!("📤 Test response sent successfully.");
                    }

                    return;
                }

                // =================================
                // UNKNOWN MESSAGE
                // =================================

                println!("⚠️ Unknown message received from peer.");
            }

            Ok(_) => {
                println!("⚠️ Peer closed the connection.");
            }

            Err(error) => {
                println!("❌ Failed to receive data: {}", error);
            }
        }
    }

    // =================================
    // SEND BLOCKCHAIN
    // =================================

    fn send_blockchain(stream: &mut TcpStream, blockchain_file: &str) {
        println!("📦 Preparing blockchain for peer...");

        // =================================
        // LOAD BLOCKCHAIN
        // =================================

        let blockchain = match Blockchain::load_from_file(blockchain_file) {
            Ok(blockchain) => blockchain,

            Err(error) => {
                println!("❌ Failed to load blockchain: {}", error);

                return;
            }
        };

        // =================================
        // VALIDATE
        // =================================

        if !blockchain.is_chain_valid() {
            println!("❌ Local blockchain is invalid. Not sending.");

            return;
        }

        // =================================
        // SERIALIZE
        // =================================

        match serde_json::to_vec(&blockchain) {
            Ok(data) => {
                if let Err(error) = stream.write_all(&data) {
                    println!("❌ Failed to send blockchain: {}", error);

                    return;
                }

                let _ = stream.shutdown(Shutdown::Write);

                println!("📤 Blockchain sent successfully.");

                println!("📊 Blocks sent: {}", blockchain.block_count());

                println!("💰 Total APXS supply sent: {}", blockchain.total_supply);
            }

            Err(error) => {
                println!("❌ Failed to encode blockchain: {}", error);
            }
        }
    }
}
