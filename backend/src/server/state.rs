use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::sync::Arc;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppState {
    pub fn init() -> Result<Arc<Self>> {
        let state = Arc::new(Self {
            connection_pool: invoicer::database::init()?,
        });
        log::info!("Created new state object");

        Ok(state)
    }
}
