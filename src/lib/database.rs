use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> Result<PgConnection> {
    let _ = dotenv();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let conn = PgConnection::establish(&database_url).context("Error connecting to database")?;

    Ok(conn)
}
