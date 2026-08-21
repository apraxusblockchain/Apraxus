mod transaction;
mod wallet;
mod block;
mod blockchain;
mod network;
mod api;

use blockchain::{
    Blockchain,
    APXS_MAX_SUPPLY,
};

use network::Network;
use wallet::Wallet;

// =================================
// APXS DISPLAY HELPERS
// =================================
//
// Blockchain ke andar amounts atomic units
// mein store hote hain.
//
// 1 APXS = 100,000,000 atomic units.
//

const APXS_UNITS: u64 = 100_000_000;

fn apxs(amount: u64) -> String {
    let whole = amount / APXS_UNITS;
    let fraction = amount % APXS_UNITS;

    if fraction == 0 {
        format!("{}.00000000", whole)
    } else {
        format!(
            "{}.{:08}",
            whole,
            fraction
        )
    }
}

// =================================
// LOAD OR CREATE WALLET
// =================================

fn load_or_create_wallet(
    path: &str,
) -> Result<Wallet, String> {

    if std::path::Path::new(path).exists() {

        println!(
            "🔐 Loading wallet: {}",
            path
        );

        Wallet::load_from_file(path)

    } else {

        println!(
            "🆕 Creating new wallet: {}",
            path
        );

        let wallet = Wallet::new();

        wallet.save_to_file(path)?;

        println!(
            "✅ Wallet saved."
        );

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

    let args: Vec<String> =
        std::env::args().collect();

    let port =
        if args.len() > 1 {
            args[1].clone()
        } else {
            "7000".to_string()
        };

    let peer_port =
        if args.len() > 2
            && args[2] != "0"
        {
            Some(args[2].clone())
        } else {
            None
        };

    let mode =
        if args.len() > 3 {
            args[3].clone()
        } else {
            "normal".to_string()
        };

    // =================================
    // NODE CONFIGURATION
    // =================================

    let node_address =
        format!(
            "127.0.0.1:{}",
            port
        );

    let blockchain_file =
        format!(
            "apraxus_chain_{}.json",
            port
        );

    let alice_wallet_file =
        "apxs_wallet_alice.dat";

    let bob_wallet_file =
        "apxs_wallet_bob.dat";

    // =================================
    // API PORT
    // =================================
    //
    // Node 7000 -> API 8000
    // Node 7001 -> API 8001
    //
    // Isse multiple local nodes
    // ek saath chal sakte hain.
    //

    let node_port: u16 =
    port.parse().unwrap_or(7000);

let api_port: u16 =
    std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| node_port.saturating_add(1000));

    println!(
        "\n🚀 Starting Apraxus node..."
    );

    println!(
        "🌐 Node address: {}",
        node_address
    );

    println!(
        "📂 Blockchain file: {}",
        blockchain_file
    );

    println!(
        "🌐 API port: {}",
        api_port
    );

    // =================================
    // START P2P NETWORK
    // =================================

    let network =
        Network::new(
            &node_address,
            &blockchain_file,
        );

    let network_thread =
        std::thread::spawn(move || {

            if let Err(error) =
                network.start()
            {
                println!(
                    "❌ Network error: {}",
                    error
                );
            }
        });

    // =================================
    // START API SERVER
    // =================================
    //
    // API alag thread mein chalegi.
    //
    // Example:
    //
    // Node 7000 -> http://localhost:8000
    // Node 7001 -> http://localhost:8001
    //

    let api_blockchain_file =
        blockchain_file.clone();

    std::thread::spawn(move || {

        let runtime =
            match tokio::runtime::Runtime::new() {

                Ok(runtime) =>
                    runtime,

                Err(error) => {

                    println!(
                        "❌ Failed to start API runtime: {}",
                        error
                    );

                    return;
                }
            };

        runtime.block_on(async move {

            if let Err(error) =
                api::start_api(
                    api_blockchain_file,
                    api_port,
                ).await
            {
                println!(
                    "❌ API server error: {}",
                    error
                );
            }

        });
    });

    // Give P2P + API a moment to start.

    std::thread::sleep(
        std::time::Duration::from_millis(500)
    );

    // =================================
    // CHECK BLOCKCHAIN
    // =================================

    let blockchain_exists =
        std::path::Path::new(
            &blockchain_file
        ).exists();

    // =================================
    // EXTEND MODE
    // =================================

    if mode == "extend" {

        println!(
            "\n🔧 EXTEND MODE"
        );

        // =================================
        // LOAD BLOCKCHAIN
        // =================================

        let mut blockchain =
            match Blockchain::load_from_file(
                &blockchain_file,
            ) {

                Ok(blockchain) =>
                    blockchain,

                Err(error) => {

                    println!(
                        "❌ Could not load blockchain: {}",
                        error
                    );

                    return;
                }
            };

        println!(
            "📂 Existing blocks: {}",
            blockchain.block_count()
        );

        // =================================
        // VALIDATE
        // =================================

        if !blockchain.is_chain_valid() {

            println!(
                "❌ Existing blockchain is invalid."
            );

            return;
        }

        println!(
            "🔐 Existing chain valid: true"
        );

        // =================================
        // LOAD ALICE
        // =================================

        let alice =
            match Wallet::load_from_file(
                alice_wallet_file
            ) {

                Ok(wallet) =>
                    wallet,

                Err(error) => {

                    println!(
                        "❌ Could not load Alice wallet: {}",
                        error
                    );

                    return;
                }
            };

        // =================================
        // LOAD BOB
        // =================================

        let bob =
            match Wallet::load_from_file(
                bob_wallet_file
            ) {

                Ok(wallet) =>
                    wallet,

                Err(error) => {

                    println!(
                        "❌ Could not load Bob wallet: {}",
                        error
                    );

                    return;
                }
            };

        // =================================
        // WALLET INFORMATION
        // =================================

        println!(
            "\n👛 Wallet information:"
        );

        println!(
            "Alice: {}",
            alice.address
        );

        println!(
            "Bob:   {}",
            bob.address
        );

        // =================================
        // CURRENT BALANCES
        // =================================

        println!(
            "\n💰 Current balances:"
        );

        println!(
            "Alice: {} APXS",
            apxs(
                blockchain.balance_of(
                    &alice.address
                )
            )
        );

        println!(
            "Bob:   {} APXS",
            apxs(
                blockchain.balance_of(
                    &bob.address
                )
            )
        );

        // =================================
        // CREATE EXTENSION TRANSACTION
        // =================================

        println!(
            "\n📝 Creating extension transaction..."
        );

        println!(
            "Alice -> Bob : 10 APXS"
        );

        let transaction =
            alice.create_transaction(
                &bob.address,

                // 10 APXS
                10 * APXS_UNITS,

                2,
            );

        println!(
            "🔐 Signature valid: {}",
            transaction.verify_signature()
        );

        // =================================
        // ADD TRANSACTION
        // =================================

        let accepted =
            blockchain.add_transaction(
                transaction.clone()
            );

        println!(
            "Transaction accepted: {}",
            accepted
        );

        // =================================
        // MINE
        // =================================

        if accepted {

            println!(
                "\n⛏️ Creating extension block..."
            );

            blockchain
                .mine_pending_transactions();
        }

        // =================================
        // VALIDATE
        // =================================

        println!(
            "\n🔐 Chain valid: {}",
            blockchain.is_chain_valid()
        );

        println!(
            "📊 New block count: {}",
            blockchain.block_count()
        );

        // =================================
        // FINAL BALANCES
        // =================================

        println!(
            "\n💰 Final balances:"
        );

        println!(
            "Alice: {} APXS",
            apxs(
                blockchain.balance_of(
                    &alice.address
                )
            )
        );

        println!(
            "Bob:   {} APXS",
            apxs(
                blockchain.balance_of(
                    &bob.address
                )
            )
        );

        println!(
            "Fee pool: {} APXS",
            apxs(
                blockchain.fee_pool_balance()
            )
        );

        // =================================
        // SAVE
        // =================================

        match blockchain.save_to_file(
            &blockchain_file
        ) {

            Ok(()) => {

                println!(
                    "✅ Extended blockchain saved successfully."
                );
            }

            Err(error) => {

                println!(
                    "❌ Failed to save extended blockchain: {}",
                    error
                );

                return;
            }
        }

        println!(
            "\n================================="
        );

        println!(
            "        APRAXUS EXTENDED"
        );

        println!(
            "================================="
        );

        return;
    }

    // =================================
    // NORMAL MODE
    // =================================

    if mode != "normal" {

        println!(
            "❌ Unknown mode: {}",
            mode
        );

        println!(
            "Available modes:"
        );

        println!(
            "  normal"
        );

        println!(
            "  extend"
        );

        return;
    }

    // =================================
    // CREATE NEW BLOCKCHAIN
    // =================================

    if !blockchain_exists {

        println!(
            "\n🆕 No existing blockchain found."
        );

        println!(
            "⛓️ Creating initial Apraxus blockchain..."
        );

        // =================================
        // LOAD / CREATE ALICE
        // =================================

        let alice =
            match load_or_create_wallet(
                alice_wallet_file
            ) {

                Ok(wallet) =>
                    wallet,

                Err(error) => {

                    println!(
                        "❌ Failed to load Alice wallet: {}",
                        error
                    );

                    return;
                }
            };

        // =================================
        // LOAD / CREATE BOB
        // =================================

        let bob =
            match load_or_create_wallet(
                bob_wallet_file
            ) {

                Ok(wallet) =>
                    wallet,

                Err(error) => {

                    println!(
                        "❌ Failed to load Bob wallet: {}",
                        error
                    );

                    return;
                }
            };

        // =================================
        // CREATE BLOCKCHAIN
        // =================================

        let mut blockchain =
            Blockchain::new();

        // =================================
        // CREATE TOTAL APXS SUPPLY
        // =================================
        //
        // APXS_MAX_SUPPLY atomic units mein hai.
        //
        // 1 APXS =
        // 100,000,000 atomic units.
        //
        // Supply genesis allocation mein
        // ONLY ONCE create hoti hai.
        //

        blockchain.add_genesis_balance(
            alice.address.clone(),
            APXS_MAX_SUPPLY,
        );

        println!(
            "\n🪙 APXS supply created."
        );

        println!(
            "Created: {} APXS",
            apxs(blockchain.total_supply)
        );

        println!(
            "Maximum: {} APXS",
            apxs(APXS_MAX_SUPPLY)
        );

        println!(
            "Total APXS supply: {} / {}",
            apxs(blockchain.total_supply),
            apxs(APXS_MAX_SUPPLY)
        );

        // =================================
        // WALLET ADDRESSES
        // =================================

        println!(
            "\n👛 Wallet addresses:"
        );

        println!(
            "Alice: {}",
            alice.address
        );

        println!(
            "Bob:   {}",
            bob.address
        );

        // =================================
        // INITIAL BALANCES
        // =================================

        println!(
            "\n💰 Initial balances:"
        );

        println!(
            "Alice: {} APXS",
            apxs(
                blockchain.balance_of(
                    &alice.address
                )
            )
        );

        println!(
            "Bob:   {} APXS",
            apxs(
                blockchain.balance_of(
                    &bob.address
                )
            )
        );

        // =================================
        // CREATE TRANSACTION
        // =================================

        println!(
            "\n📝 Creating signed transaction..."
        );

        println!(
            "Alice -> Bob : 25 APXS"
        );

        let transaction =
            alice.create_transaction(
                &bob.address,

                // 25 APXS
                25 * APXS_UNITS,

                0,
            );

        println!(
            "🔐 Signature valid: {}",
            transaction.verify_signature()
        );

        // =================================
        // ADD TO TRANSACTION POOL
        // =================================

        println!(
            "\n📥 Adding transaction to pool..."
        );

        let accepted =
            blockchain.add_transaction(
                transaction.clone()
            );

        println!(
            "Transaction accepted: {}",
            accepted
        );

        // =================================
        // BROADCAST
        // =================================

        if accepted {

            if let Some(peer_port) =
                peer_port.clone()
            {

                let peer_address =
                    format!(
                        "127.0.0.1:{}",
                        peer_port
                    );

                println!(
                    "\n📡 Broadcasting transaction to {}...",
                    peer_address
                );

                let broadcast_network =
                    Network::new(
                        &node_address,
                        &blockchain_file,
                    );

                match broadcast_network
                    .broadcast_transaction(
                        &peer_address,
                        &transaction,
                    )
                {

                    Ok(()) => {

                        println!(
                            "✅ Transaction broadcast completed."
                        );
                    }

                    Err(error) => {

                        println!(
                            "❌ Transaction broadcast failed: {}",
                            error
                        );
                    }
                }

            } else {

                println!(
                    "\nℹ️ No peer configured."
                );

                println!(
                    "Transaction will remain local."
                );
            }
        }

        // =================================
        // MINE BLOCK
        // =================================

        println!(
            "\n⛏️ Creating block..."
        );

        blockchain
            .mine_pending_transactions();

        // =================================
        // VALIDATE
        // =================================

        println!(
            "\n🔐 Chain valid: {}",
            blockchain.is_chain_valid()
        );

        // =================================
        // SAVE
        // =================================

        println!(
            "\n💾 Saving blockchain..."
        );

        match blockchain.save_to_file(
            &blockchain_file
        ) {

            Ok(()) => {

                println!(
                    "✅ Blockchain saved successfully."
                );
            }

            Err(error) => {

                println!(
                    "❌ Failed to save blockchain: {}",
                    error
                );

                return;
            }
        }

        // =================================
        // FINAL BALANCES
        // =================================

        println!(
            "\n💰 Final balances:"
        );

        println!(
            "Alice: {} APXS",
            apxs(
                blockchain.balance_of(
                    &alice.address
                )
            )
        );

        println!(
            "Bob:   {} APXS",
            apxs(
                blockchain.balance_of(
                    &bob.address
                )
            )
        );

        println!(
            "Fee pool: {} APXS",
            apxs(
                blockchain.fee_pool_balance()
            )
        );

        println!(
            "📊 Total blocks: {}",
            blockchain.block_count()
        );

        println!(
            "\n================================="
        );

        println!(
            "        APRAXUS NODE READY"
        );

        println!(
            "================================="
        );

        println!(
            "🌐 API: http://127.0.0.1:{}",
            api_port
        );

        // Keep node alive.
        let _ =
            network_thread.join();

        return;
    }

    // =================================
    // EXISTING BLOCKCHAIN
    // =================================

    println!(
        "\n📂 Existing blockchain detected."
    );

    let blockchain =
        match Blockchain::load_from_file(
            &blockchain_file
        ) {

            Ok(blockchain) =>
                blockchain,

            Err(error) => {

                println!(
                    "❌ Failed to load blockchain: {}",
                    error
                );

                return;
            }
        };

    println!(
        "✅ Blockchain loaded successfully."
    );

    println!(
        "📊 Blocks: {}",
        blockchain.block_count()
    );

    println!(
        "💰 Total APXS supply: {} / {}",
        apxs(blockchain.total_supply),
        apxs(APXS_MAX_SUPPLY)
    );

    println!(
        "🔐 Chain valid: {}",
        blockchain.is_chain_valid()
    );

    // =================================
    // LOAD PERSISTENT WALLETS
    // =================================

    println!(
        "\n💳 Persistent wallets:"
    );

    let alice =
        match Wallet::load_from_file(
            alice_wallet_file
        ) {

            Ok(wallet) =>
                Some(wallet),

            Err(error) => {

                println!(
                    "⚠️ Alice wallet unavailable: {}",
                    error
                );

                None
            }
        };

    let bob =
        match Wallet::load_from_file(
            bob_wallet_file
        ) {

            Ok(wallet) =>
                Some(wallet),

            Err(error) => {

                println!(
                    "⚠️ Bob wallet unavailable: {}",
                    error
                );

                None
            }
        };

    // =================================
    // DISPLAY ALICE
    // =================================

    if let Some(alice) =
        &alice
    {

        println!(
            "Alice: {}",
            alice.address
        );

        println!(
            "Alice balance: {} APXS",
            apxs(
                blockchain.balance_of(
                    &alice.address
                )
            )
        );
    }

    // =================================
    // DISPLAY BOB
    // =================================

    if let Some(bob) =
        &bob
    {

        println!(
            "Bob: {}",
            bob.address
        );

        println!(
            "Bob balance: {} APXS",
            apxs(
                blockchain.balance_of(
                    &bob.address
                )
            )
        );
    }

    // =================================
    // CONNECT / SYNC WITH PEER
    // =================================

    if let Some(peer_port) =
        peer_port
    {

        let peer_address =
            format!(
                "127.0.0.1:{}",
                peer_port
            );

        println!(
            "\n🔗 Connecting to peer {}...",
            peer_address
        );

        let peer_network =
            Network::new(
                &node_address,
                &blockchain_file,
            );

        if let Err(error) =
            peer_network.connect_to_peer(
                &peer_address
            )
        {

            println!(
                "❌ Peer connection failed: {}",
                error
            );
        }
    }

    // =================================
    // NODE READY
    // =================================

    println!(
        "\n================================="
    );

    println!(
        "        APRAXUS NODE READY"
    );

    println!(
        "================================="
    );

    println!(
        "🌐 API: http://127.0.0.1:{}",
        api_port
    );

    // =================================
    // KEEP NODE RUNNING
    // =================================

    let _ =
        network_thread.join();
}