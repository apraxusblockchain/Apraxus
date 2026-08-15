mod transaction;
mod wallet;
mod block;
mod blockchain;

use blockchain::Blockchain;
use wallet::Wallet;

fn main() {
    const BLOCKCHAIN_FILE: &str = "apraxus_chain.json";

    println!("=================================");
    println!("        APRAXUS BLOCKCHAIN");
    println!("=================================");

    // Create two wallets.
    let alice = Wallet::new();
    let bob = Wallet::new();

    println!("\nAlice address:");
    println!("{}", alice.address);

    println!("\nBob address:");
    println!("{}", bob.address);

    // Create blockchain.
    let mut blockchain = Blockchain::new();

    // Development-only genesis allocation.
    blockchain.add_genesis_balance(
        alice.address.clone(),
        100,
    );

    println!("\nInitial balances:");
    println!(
        "Alice: {} APXV",
        blockchain.balance_of(&alice.address)
    );
    println!(
        "Bob:   {} APXV",
        blockchain.balance_of(&bob.address)
    );

    // Alice creates and signs a transaction.
    println!("\nCreating signed transaction...");
    println!("Alice -> Bob : 25 APXV");

    let transaction = alice.create_transaction(
        &bob.address,
        25,
        0,
    );

    println!(
        "Signature valid: {}",
        transaction.verify_signature()
    );

    // Add transaction to pending pool.
    println!("\nAdding transaction to pool...");

    let accepted =
        blockchain.add_transaction(transaction);

    println!(
        "Transaction accepted: {}",
        accepted
    );

    // Create a block from pending transactions.
    println!("\nCreating block...");

    blockchain.mine_pending_transactions();

    // Validate the chain.
    println!(
        "\nChain valid: {}",
        blockchain.is_chain_valid()
    );

    // Save blockchain to disk.
    println!(
        "\nSaving blockchain to {}...",
        BLOCKCHAIN_FILE
    );

    match blockchain.save_to_file(BLOCKCHAIN_FILE) {
        Ok(()) => println!("✅ Blockchain saved successfully."),
        Err(error) => {
            println!("❌ Failed to save blockchain: {}", error);
            return;
        }
    }

    // Display final balances.
    println!("\nFinal balances:");

    println!(
        "Alice: {} APXV",
        blockchain.balance_of(&alice.address)
    );

    println!(
        "Bob:   {} APXV",
        blockchain.balance_of(&bob.address)
    );

    println!(
        "Total blocks: {}",
        blockchain.block_count()
    );

    // Load blockchain back from disk.
    println!(
        "\nLoading blockchain from {}...",
        BLOCKCHAIN_FILE
    );

    match Blockchain::load_from_file(BLOCKCHAIN_FILE) {
        Ok(loaded_blockchain) => {
            println!("✅ Blockchain loaded successfully.");

            println!(
                "Loaded blocks: {}",
                loaded_blockchain.block_count()
            );

            println!(
                "Loaded chain valid: {}",
                loaded_blockchain.is_chain_valid()
            );
        }

        Err(error) => {
            println!(
                "❌ Failed to load blockchain: {}",
                error
            );
        }
    }

    println!("=================================");
}