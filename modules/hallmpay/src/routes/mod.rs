use axum::Router;

use crate::db::DbPool;

pub mod adapter;
pub mod api;
pub mod auth;
pub mod cli;
pub mod doc;
pub mod health;
pub mod invoice;
pub mod model;
pub mod usage;

// Build the API router without binding state; it will be provided at the top level.
pub fn create_api_router() -> Router<DbPool> {
    Router::new()
        .nest("/adapter", adapter::router())
        .nest("/auth", auth::router())
        .nest("/api", api::router())
        .nest("/doc", doc::route())
        .nest("/cli", cli::router())
        .nest("/usage", usage::router())
        .nest("/invoice", invoice::router())
}
