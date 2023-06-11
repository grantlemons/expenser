use axum::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Information on version and other fields set in the cargo manifest
pub(crate) struct CrateInfo {
    name: &'static str,
    authors: Vec<&'static str>,
    version: &'static str,
    description: &'static str,
    license: &'static str,
    repository: &'static str,
}

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
