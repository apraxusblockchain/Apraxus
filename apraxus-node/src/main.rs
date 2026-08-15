mod transaction;
mod wallet;
mod block;
mod blockchain;

use blockchain::Blockchain;
use wallet::Wallet;

fn main() {
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

    // Alice signs a transaction.
    println!("\nCreating signed transaction...");
    println!("Alice -> Bob : 25 APXV");

    let transaction =
        alice.create_transaction(
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

    // Display final state.
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

    // Validate the entire blockchain.
    println!(
        "Chain valid: {}",
        blockchain.is_chain_valid()
    );

    println!("=================================");
}
