use super::types::CrateInfo;
use axum::Json;

/// Healthcheck handler
///
/// Returns `Healthy!` if healthy
pub(crate) async fn health() -> &'static str {
    log::info!("Request made to health endpoint");

    "Healthy!"
}

/// Crate information handler used to get information on the server
///
/// Uses [`CrateInfo`] struct
pub(crate) async fn info() -> Json<CrateInfo> {
    log::info!("Request made to info endpoint");

    let res = CrateInfo {
        name: env!("CARGO_PKG_NAME"),
        authors: env!("CARGO_PKG_AUTHORS").split(',').collect(),
        version: env!("CARGO_PKG_VERSION"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        license: env!("CARGO_PKG_LICENSE"),
        repository: env!("CARGO_PKG_REPOSITORY"),
    };

    Json(res)
}
