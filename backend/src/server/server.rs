use anyhow::Result;
use axum::Router;

#[allow(unused_imports)]
use invoicer::*;

mod handlers {
    /// Handlers for server info and health check
    mod info;

    pub(crate) use info::*;
}
mod logger;
mod state;
pub use state::AppState;

const PORT: u16 = 3000;
const LOCALHOST: [u8; 4] = [0, 0, 0, 0];

fn api() -> Result<Router> {
    #[allow(unused_imports)]
    use axum::routing::{delete, get, post, put};
    use handlers::*;

    let router = Router::new()
        .route("/health", get(health)) // Health check
        .route("/info", get(info))
        .with_state(AppState::init()?);

    Ok(router)
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;

    let app = Router::new().nest("/api", api()?);

    let addr = std::net::SocketAddr::from((LOCALHOST, PORT));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    log::info!("Server running on port {}", PORT);

    Ok(())
}
