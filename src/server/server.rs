use anyhow::{Context, Result};
use axum::Router;
use std::net::SocketAddr;

#[allow(unused_imports)]
use invoice::*;

mod handlers {
    /// Handlers for server info and health check
    mod info;

    pub(crate) use info::*;
}

const PORT: u16 = 3000;
const LOCALHOST: [u8; 4] = [0, 0, 0, 0];

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from((LOCALHOST, PORT));
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await?;

    Ok(())
}

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

/// Logger configuration using [`fern`]
fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(
            fern::log_file(format!(
                "/logs/{}.log",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M]")
            ))
            .context("Unable to open log file")?,
        )
        .apply()
        .context("Failed to dispatch logger")?;
    Ok(())
}
