use anyhow::Result;
use axum::Router;

#[allow(unused_imports)]
use expenser::*;

mod handlers {
    mod access;
    /// Handlers for server info and health check
    mod info;
    mod line_items;
    mod proof;
    mod reports;
    mod types;
    mod users;

    pub(crate) use access::*;
    pub(crate) use info::*;
    pub(crate) use line_items::*;
    pub(crate) use proof::*;
    pub(crate) use reports::*;
    pub(crate) use users::*;
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
        .route("/reports", post(create_report))
        .route(
            "/reports/:id",
            get(get_report).put(update_report).delete(delete_report),
        )
        .route(
            "/reports/:report_id/items",
            get(get_line_items_by_report)
                .post(create_line_item)
                .delete(clear_line_items),
        )
        .route(
            "/reports/:report_id/items/:id",
            get(get_line_item)
                .put(update_line_item)
                .delete(delete_line_item),
        )
        .route(
            "/reports/:report_id/access",
            get(get_access_by_report)
                .post(create_access)
                .delete(clear_access),
        )
        .route(
            "/reports/:report_id/access/:id",
            get(get_access).put(update_access).delete(delete_access),
        )
        .route(
            "/reports/:report_id/proof",
            get(get_proof_by_report)
                .post(create_proof)
                .delete(clear_proof),
        )
        .route(
            "/reports/:report_id/proof/:id",
            get(get_proof).put(update_proof).delete(delete_proof),
        )
        .route("/users", post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route(
            "/users/:id/pfp",
            get(get_profile_picture).put(update_profile_picture),
        )
        .route("/users/:id/password", put(update_password))
        .route("/users/:id/reports", get(get_reports_by_owner))
        .route("/users/:id/reports/access", get(get_reports_by_view_access))
        .with_state(AppState::init()?);

    Ok(router)
}

fn load_dotenv() {
    match dotenvy::dotenv() {
        Ok(_) => log::info!("Loaded info from dotenv"),
        Err(err) => log::error!("Unable to load info from dotenv: \"{}\"", err),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::setup()?;
    load_dotenv();

    let app = Router::new().nest("/api", api()?);
    let addr = std::net::SocketAddr::from((LOCALHOST, PORT));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    log::info!("Server running on port {}", PORT);

    Ok(())
}
