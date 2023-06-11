use anyhow::Result;
use axum::Router;

#[allow(unused_imports)]
use invoice::*;

mod handlers {
    /// Handlers for server info and health check
    mod info;

    pub(crate) use info::*;
}
mod logger;

const PORT: u16 = 3000;
const LOCALHOST: [u8; 4] = [0, 0, 0, 0];

fn app() -> Router {
    #[allow(unused_imports)]
    use axum::routing::{delete, get, post, put};
    use handlers::*;

    let routes: Router = Router::new()
        .route("/health", get(health)) // Health check
        .route("/info", get(info));

    Router::new().nest("/api", routes)
    // .layer(axum::middleware::from_fn(auth))
    // .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;

    let addr = std::net::SocketAddr::from((LOCALHOST, PORT));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await?;

    Ok(())
}
