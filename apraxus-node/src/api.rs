use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use serde_json::{json, Value};

use sha2::{Digest, Sha256};

use crate::blockchain::{APXS_DEFAULT_FEE, APXS_MAX_SUPPLY, Blockchain};

use std::sync::Arc;

// =================================
// API STATE
// =================================

#[derive(Clone)]
pub struct ApiState {
    pub blockchain_file: Arc<String>,
}

// =================================
// START API SERVER
// =================================

pub async fn start_api(
    blockchain_file: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = ApiState {
        blockchain_file: Arc::new(blockchain_file),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/token", get(token_info))
        .route("/blockchain", get(blockchain_info))
        .route("/balance/{address}", get(balance))
        .route("/blocks", get(blocks))
        .route("/block/{height}", get(block))
        .route("/transaction/{hash}", get(transaction))
        .with_state(state);

    let address = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("🌐 APXS API listening on http://{}", address);

    axum::serve(listener, app).await?;

    Ok(())
}

// =================================
// ROOT
// =================================

async fn root() -> Json<Value> {
    Json(json!({
        "name": "Apraxus",
        "symbol": "APXS",
        "status": "online",
        "message": "Welcome to the Apraxus API"
    }))
}

// =================================
// HEALTH
// =================================

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "network": "Apraxus",
        "token": "APXS"
    }))
}

// =================================
// TOKEN INFORMATION
// =================================

async fn token_info() -> Json<Value> {
    Json(json!({
        "name": "Apraxus",
        "symbol": "APXS",
        "decimals": 8,
        "max_supply": APXS_MAX_SUPPLY,
        "default_fee": APXS_DEFAULT_FEE
    }))
}

// =================================
// BLOCKCHAIN INFORMATION
// =================================

async fn blockchain_info(
    State(state): State<ApiState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let blockchain = Blockchain::load_from_file(state.blockchain_file.as_str())
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;

    Ok(Json(json!({
        "blocks": blockchain.block_count(),
        "total_supply": blockchain.total_supply,
        "max_supply": APXS_MAX_SUPPLY,
        "valid": blockchain.is_chain_valid()
    })))
}

// =================================
// BALANCE
// =================================

async fn balance(
    State(state): State<ApiState>,
    Path(address): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let blockchain = Blockchain::load_from_file(state.blockchain_file.as_str())
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;

    let amount = blockchain.balance_of(&address);

    Ok(Json(json!({
        "address": address,
        "balance": amount,
        "symbol": "APXS"
    })))
}

// =================================
// TRANSACTION HASH
// =================================
//
// Transaction IDs are calculated from
// the transaction's canonical fields.
//
// This does NOT modify the blockchain
// storage format.

fn transaction_hash(
    sender: &str,
    recipient: &str,
    amount: u64,
    nonce: u64,
    fee: u64,
) -> String {
    let input = format!(
        "{}{}{}{}{}",
        sender, recipient, amount, nonce, fee
    );

    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}

// =================================
// BLOCK LIST
// =================================

async fn blocks(
    State(state): State<ApiState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let blockchain = Blockchain::load_from_file(state.blockchain_file.as_str())
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;

    let blocks = blockchain
        .blocks
        .iter()
        .map(|block| {
            json!({
                "index": block.index,
                "timestamp": block.timestamp,
                "previous_hash": block.previous_hash,
                "hash": block.hash,
                "transactions": block.transactions.len()
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "network": "Apraxus",
        "symbol": "APXS",
        "height": blockchain.block_count(),
        "blocks": blocks
    })))
}

// =================================
// SINGLE BLOCK
// =================================

async fn block(
    State(state): State<ApiState>,
    Path(height): Path<u64>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let blockchain = Blockchain::load_from_file(state.blockchain_file.as_str())
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;

    let block = match blockchain.blocks.iter().find(|block| block.index == height) {
        Some(block) => block,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Block {} not found", height),
            ));
        }
    };

    let transactions = block
        .transactions
        .iter()
        .map(|tx| {
            let hash = transaction_hash(
                &tx.sender,
                &tx.recipient,
                tx.amount,
                tx.nonce,
                tx.fee,
            );

            json!({
                "hash": hash,
                "sender": tx.sender,
                "recipient": tx.recipient,
                "amount": tx.amount,
                "fee": tx.fee,
                "nonce": tx.nonce
            })
        })
        .collect::<Vec<_>>();

    Ok(Json(json!({
        "network": "Apraxus",
        "symbol": "APXS",
        "index": block.index,
        "timestamp": block.timestamp,
        "previous_hash": block.previous_hash,
        "hash": block.hash,
        "transactions": transactions
    })))
}

// =================================
// TRANSACTION LOOKUP
// =================================

async fn transaction(
    State(state): State<ApiState>,
    Path(hash): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let blockchain = Blockchain::load_from_file(state.blockchain_file.as_str())
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error))?;

    for block in &blockchain.blocks {
        for tx in &block.transactions {
            let tx_hash = transaction_hash(
                &tx.sender,
                &tx.recipient,
                tx.amount,
                tx.nonce,
                tx.fee,
            );

            if tx_hash == hash {
                return Ok(Json(json!({
                    "network": "Apraxus",
                    "symbol": "APXS",
                    "hash": tx_hash,
                    "status": "confirmed",
                    "block": block.index,
                    "block_hash": block.hash,
                    "timestamp": block.timestamp,
                    "sender": tx.sender,
                    "recipient": tx.recipient,
                    "amount": tx.amount,
                    "fee": tx.fee,
                    "nonce": tx.nonce
                })));
            }
        }
    }

    Err((
        StatusCode::NOT_FOUND,
        format!("Transaction {} not found", hash),
    ))
}
