use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl AppState {
    pub fn init() -> Result<Self> {
        let state = Self {
            connection_pool: Arc::new(expenser::database::init()?),
        };
        log::info!("Created new state object");

        Ok(state)
    }

    pub fn get_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, axum::http::StatusCode> {
        match expenser::database::get_connection(&self.connection_pool) {
            Ok(res) => Ok(res),
            Err(e) => {
                log::error!("{e}");
                Err(axum::http::StatusCode::GATEWAY_TIMEOUT)
            }
        }
    }
}
