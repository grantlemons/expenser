use anyhow::{Context, Result};
use diesel::{
    pg::{Pg, PgConnection},
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run migrations on database");
}

pub fn get_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
    pool.get().context("Unable to get connection from pool")
}

pub fn pool() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let _ = dotenv();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .context("Unable to establish connection pool")
}

pub fn init() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let pool = pool()?;
    let conn = &mut get_connection(&pool)?;

    run_migrations(conn);

    Ok(pool)
}
