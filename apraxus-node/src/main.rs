mod api;
mod block;
mod blockchain;
mod network;
mod transaction;
mod wallet;

use blockchain::{APXS_DEFAULT_FEE, APXS_MAX_SUPPLY, Blockchain};

use network::Network;
use wallet::Wallet;

// =================================
// APXS DISPLAY
// =================================

const APXS_UNITS: u64 = 100_000_000;

fn apxs(amount: u64) -> String {
    let whole = amount / APXS_UNITS;
    let fraction = amount % APXS_UNITS;

    if fraction == 0 {
        format!("{}.00000000", whole)
    } else {
        format!("{}.{:08}", whole, fraction)
    }
}

// =================================
// LOAD / CREATE WALLET
// =================================

fn load_or_create_wallet(path: &str) -> Result<Wallet, String> {
    if std::path::Path::new(path).exists() {
        println!("🔐 Loading wallet: {}", path);
        Wallet::load_from_file(path)
    } else {
        println!("🆕 Creating new wallet: {}", path);

        let wallet = Wallet::new();

        wallet.save_to_file(path)?;

        println!("✅ Wallet saved.");

        Ok(wallet)
    }
}

// =================================
// MAIN
// =================================

fn main() {
    // =================================
    // COMMAND LINE ARGUMENTS
    // =================================

    let args: Vec<String> = std::env::args().collect();

    let node_port = if args.len() > 1 {
        args[1].parse::<u16>().unwrap_or(7000)
    } else {
        7000
    };

    let peer_port = if args.len() > 2 && args[2] != "0" {
        Some(args[2].parse::<u16>().unwrap_or(0))
    } else {
        None
    };

    let mode = if args.len() > 3 {
        args[3].clone()
    } else {
        "normal".to_string()
    };

    // =================================
    // NODE CONFIGURATION
    // =================================

    // Local development:
    // 127.0.0.1
    //
    // Render:
    // API is exposed separately by api.rs
    // through 0.0.0.0:$PORT.

    let node_host = std::env::var("NODE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let node_address = format!("{}:{}", node_host, node_port);

    let blockchain_file = std::env::var("BLOCKCHAIN_FILE")
        .unwrap_or_else(|_| format!("apraxus_chain_{}.json", node_port));

    let alice_wallet_file =
        std::env::var("ALICE_WALLET_FILE").unwrap_or_else(|_| "apxs_wallet_alice.dat".to_string());

    let bob_wallet_file =
        std::env::var("BOB_WALLET_FILE").unwrap_or_else(|_| "apxs_wallet_bob.dat".to_string());

    // =================================
    // API PORT
    // =================================
    //
    // Render automatically provides PORT.
    //
    // Local:
    // node 7000 -> API 8000
    // node 7001 -> API 8001
    //
    // Render:
    // API -> $PORT
    //

    let api_port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or_else(|| node_port.saturating_add(1000));

    println!();
    println!("=================================");
    println!("        APRAXUS STARTING");
    println!("=================================");

    println!("🌐 Node address: {}", node_address);
    println!("📂 Blockchain file: {}", blockchain_file);
    println!("🌐 API port: {}", api_port);

    // =================================
    // START P2P NETWORK
    // =================================

    let network = Network::new(&node_address, &blockchain_file);

    let network_thread = std::thread::spawn(move || {
        if let Err(error) = network.start() {
            println!("❌ Network error: {}", error);
        }
    });

    // =================================
    // START API
    // =================================

    let api_blockchain_file = blockchain_file.clone();

    std::thread::spawn(move || {
        let runtime = match tokio::runtime::Runtime::new() {
            Ok(runtime) => runtime,

            Err(error) => {
                println!("❌ Failed to start API runtime: {}", error);

                return;
            }
        };

        runtime.block_on(async move {
            if let Err(error) = api::start_api(api_blockchain_file, api_port).await {
                println!("❌ API server error: {}", error);
            }
        });
    });

    // =================================
    // GIVE SERVERS TIME TO START
    // =================================

    std::thread::sleep(std::time::Duration::from_millis(500));

    // =================================
    // BLOCKCHAIN FILE CHECK
    // =================================

    let blockchain_exists = std::path::Path::new(&blockchain_file).exists();

    // =================================
    // EXTEND MODE
    // =================================

    if mode == "extend" {
        println!();
        println!("🔧 EXTEND MODE");

        let mut blockchain = match Blockchain::load_from_file(&blockchain_file) {
            Ok(blockchain) => blockchain,

            Err(error) => {
                println!("❌ Could not load blockchain: {}", error);

                return;
            }
        };

        println!("📂 Existing blocks: {}", blockchain.block_count());

        if !blockchain.is_chain_valid() {
            println!("❌ Existing blockchain is invalid.");

            return;
        }

        println!("🔐 Existing chain valid: true");

        let alice = match Wallet::load_from_file(&alice_wallet_file) {
            Ok(wallet) => wallet,

            Err(error) => {
                println!("❌ Could not load Alice wallet: {}", error);

                return;
            }
        };

        let bob = match Wallet::load_from_file(&bob_wallet_file) {
            Ok(wallet) => wallet,

            Err(error) => {
                println!("❌ Could not load Bob wallet: {}", error);

                return;
            }
        };

        println!();
        println!("👛 Wallet information:");
        println!("Alice: {}", alice.address);
        println!("Bob:   {}", bob.address);

        println!();
        println!("💰 Current balances:");

        println!(
            "Alice: {} APXS",
            apxs(blockchain.balance_of(&alice.address))
        );

        println!("Bob:   {} APXS", apxs(blockchain.balance_of(&bob.address)));

        println!();
        println!("📝 Creating extension transaction...");

        println!("Alice -> Bob : 10 APXS");

        let transaction = alice.create_transaction(&bob.address, 10 * APXS_UNITS, 2);

        println!("🔐 Signature valid: {}", transaction.verify_signature());

        let accepted = blockchain.add_transaction(transaction.clone());

        println!("Transaction accepted: {}", accepted);

        if accepted {
            println!("\n⛏️ Creating extension block...");

            blockchain.mine_pending_transactions();
        }

        println!("\n🔐 Chain valid: {}", blockchain.is_chain_valid());

        println!("📊 New block count: {}", blockchain.block_count());

        println!();
        println!("💰 Final balances:");

        println!(
            "Alice: {} APXS",
            apxs(blockchain.balance_of(&alice.address))
        );

        println!("Bob:   {} APXS", apxs(blockchain.balance_of(&bob.address)));

        println!("Fee pool: {} APXS", apxs(blockchain.fee_pool_balance()));

        match blockchain.save_to_file(&blockchain_file) {
            Ok(()) => {
                println!("✅ Extended blockchain saved successfully.");
            }

            Err(error) => {
                println!("❌ Failed to save extended blockchain: {}", error);

                return;
            }
        }

        println!();
        println!("=================================");
        println!("        APRAXUS EXTENDED");
        println!("=================================");

        return;
    }

    // =================================
    // MODE VALIDATION
    // =================================

    if mode != "normal" {
        println!("❌ Unknown mode: {}", mode);

        println!("Available modes:");
        println!("  normal");
        println!("  extend");

        return;
    }

    // =================================
    // CREATE NEW BLOCKCHAIN
    // =================================

    if !blockchain_exists {
        println!();
        println!("🆕 No existing blockchain found.");

        println!("⛓️ Creating initial Apraxus blockchain...");

        let alice = match load_or_create_wallet(&alice_wallet_file) {
            Ok(wallet) => wallet,

            Err(error) => {
                println!("❌ Failed to load Alice wallet: {}", error);

                return;
            }
        };

        let bob = match load_or_create_wallet(&bob_wallet_file) {
            Ok(wallet) => wallet,

            Err(error) => {
                println!("❌ Failed to load Bob wallet: {}", error);

                return;
            }
        };

        let mut blockchain = Blockchain::new();

        blockchain.add_genesis_balance(alice.address.clone(), APXS_MAX_SUPPLY);

        println!();
        println!("🪙 APXS supply created.");

        println!("Created: {} APXS", apxs(blockchain.total_supply));

        println!("Maximum: {} APXS", apxs(APXS_MAX_SUPPLY));

        println!(
            "Total APXS supply: {} / {}",
            apxs(blockchain.total_supply),
            apxs(APXS_MAX_SUPPLY)
        );

        println!();
        println!("👛 Wallet addresses:");

        println!("Alice: {}", alice.address);

        println!("Bob:   {}", bob.address);

        println!();
        println!("💰 Initial balances:");

        println!(
            "Alice: {} APXS",
            apxs(blockchain.balance_of(&alice.address))
        );

        println!("Bob:   {} APXS", apxs(blockchain.balance_of(&bob.address)));

        println!();
        println!("📝 Creating signed transaction...");

        println!("Alice -> Bob : 25 APXS");

        let transaction = alice.create_transaction(&bob.address, 25 * APXS_UNITS, APXS_DEFAULT_FEE);

        println!("🔐 Signature valid: {}", transaction.verify_signature());

        println!();
        println!("📥 Adding transaction to pool...");

        let accepted = blockchain.add_transaction(transaction.clone());

        println!("Transaction accepted: {}", accepted);

        // =================================
        // BROADCAST
        // =================================

        if accepted {
            if let Some(peer_port) = peer_port {
                if peer_port != 0 {
                    let peer_address = format!("{}:{}", node_host, peer_port);

                    println!("\n📡 Broadcasting transaction to {}...", peer_address);

                    let broadcast_network = Network::new(&node_address, &blockchain_file);

                    match broadcast_network.broadcast_transaction(&peer_address, &transaction) {
                        Ok(()) => {
                            println!("✅ Transaction broadcast completed.");
                        }

                        Err(error) => {
                            println!("❌ Transaction broadcast failed: {}", error);
                        }
                    }
                }
            } else {
                println!("\nℹ️ No peer configured.");

                println!("Transaction will remain local.");
            }
        }

        // =================================
        // MINE
        // =================================

        println!("\n⛏️ Creating block...");

        blockchain.mine_pending_transactions();

        println!("\n🔐 Chain valid: {}", blockchain.is_chain_valid());

        println!("\n💾 Saving blockchain...");

        match blockchain.save_to_file(&blockchain_file) {
            Ok(()) => {
                println!("✅ Blockchain saved successfully.");
            }

            Err(error) => {
                println!("❌ Failed to save blockchain: {}", error);

                return;
            }
        }

        println!();
        println!("💰 Final balances:");

        println!(
            "Alice: {} APXS",
            apxs(blockchain.balance_of(&alice.address))
        );

        println!("Bob:   {} APXS", apxs(blockchain.balance_of(&bob.address)));

        println!("Fee pool: {} APXS", apxs(blockchain.fee_pool_balance()));

        println!("📊 Total blocks: {}", blockchain.block_count());

        println!();
        println!("=================================");
        println!("        APRAXUS NODE READY");
        println!("=================================");

        println!("🌐 API: http://127.0.0.1:{}", api_port);

        println!("🌐 API bind: 0.0.0.0:{}", api_port);

        let _ = network_thread.join();

        return;
    }

    // =================================
    // EXISTING BLOCKCHAIN
    // =================================

    println!();
    println!("📂 Existing blockchain detected.");

    let blockchain = match Blockchain::load_from_file(&blockchain_file) {
        Ok(blockchain) => blockchain,

        Err(error) => {
            println!("❌ Failed to load blockchain: {}", error);

            return;
        }
    };

    println!("✅ Blockchain loaded successfully.");

    println!("📊 Blocks: {}", blockchain.block_count());

    println!(
        "💰 Total APXS supply: {} / {}",
        apxs(blockchain.total_supply),
        apxs(APXS_MAX_SUPPLY)
    );

    println!("🔐 Chain valid: {}", blockchain.is_chain_valid());

    // =================================
    // LOAD PERSISTENT WALLETS
    // =================================

    println!();
    println!("💳 Persistent wallets:");

    let alice = match Wallet::load_from_file(&alice_wallet_file) {
        Ok(wallet) => Some(wallet),

        Err(error) => {
            println!("⚠️ Alice wallet unavailable: {}", error);

            None
        }
    };

    let bob = match Wallet::load_from_file(&bob_wallet_file) {
        Ok(wallet) => Some(wallet),

        Err(error) => {
            println!("⚠️ Bob wallet unavailable: {}", error);

            None
        }
    };

    if let Some(alice) = &alice {
        println!("Alice: {}", alice.address);

        println!(
            "Alice balance: {} APXS",
            apxs(blockchain.balance_of(&alice.address))
        );
    }

    if let Some(bob) = &bob {
        println!("Bob: {}", bob.address);

        println!(
            "Bob balance: {} APXS",
            apxs(blockchain.balance_of(&bob.address))
        );
    }

    // =================================
    // CONNECT TO PEER
    // =================================

    if let Some(peer_port) = peer_port {
        if peer_port != 0 {
            let peer_address = format!("{}:{}", node_host, peer_port);

            println!("\n🔗 Connecting to peer {}...", peer_address);

            let peer_network = Network::new(&node_address, &blockchain_file);

            if let Err(error) = peer_network.connect_to_peer(&peer_address) {
                println!("❌ Peer connection failed: {}", error);
            }
        }
    }

    // =================================
    // NODE READY
    // =================================

    println!();
    println!("=================================");
    println!("        APRAXUS NODE READY");
    println!("=================================");

    println!("🌐 API local: http://127.0.0.1:{}", api_port);

    println!("🌐 API bind: 0.0.0.0:{}", api_port);

    println!("⛓️ Blockchain: {}", blockchain_file);

    // =================================
    // KEEP NODE RUNNING
    // =================================

    let _ = network_thread.join();
}
