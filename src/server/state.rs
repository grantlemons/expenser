use anyhow::Result;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl AppState {
    pub fn init() -> Result<Arc<Self>> {
        Ok(Arc::new(Self {
            connection_pool: invoice::database::init()?,
        }))
    }
}
