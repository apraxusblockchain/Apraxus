use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
    Router,
};

use serde_json::{json, Value};

use crate::blockchain::{
    Blockchain,
    APXS_DEFAULT_FEE,
    APXS_MAX_SUPPLY,
};

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
        blockchain_file: Arc::new(
            blockchain_file
        ),
    };

    let app =
        Router::new()
            .route(
                "/",
                get(root)
            )
            .route(
                "/health",
                get(health)
            )
            .route(
                "/token",
                get(token_info)
            )
            .route(
                "/blockchain",
                get(blockchain_info)
            )
            .route(
                "/balance/{address}",
                get(balance)
            )
            .with_state(state);

    let address =
        format!(
            "0.0.0.0:{}",
            port
        );

    let listener =
        tokio::net::TcpListener::bind(
            &address
        ).await?;

    println!(
        "🌐 APXS API listening on http://{}",
        address
    );

    axum::serve(
        listener,
        app,
    ).await?;

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

    let blockchain =
        Blockchain::load_from_file(
            state.blockchain_file.as_str()
        )
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                error,
            )
        })?;

    Ok(
        Json(json!({
            "blocks": blockchain.block_count(),
            "total_supply": blockchain.total_supply,
            "max_supply": APXS_MAX_SUPPLY,
            "valid": blockchain.is_chain_valid()
        }))
    )
}

// =================================
// BALANCE
// =================================

async fn balance(
    State(state): State<ApiState>,
    Path(address): Path<String>,
) -> Result<Json<Value>, (StatusCode, String)> {

    let blockchain =
        Blockchain::load_from_file(
            state.blockchain_file.as_str()
        )
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                error,
            )
        })?;

    let amount =
        blockchain.balance_of(
            &address
        );

    Ok(
        Json(json!({
            "address": address,
            "balance": amount,
            "symbol": "APXS"
        }))
    )
}