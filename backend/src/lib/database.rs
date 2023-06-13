use anyhow::{Context, Result};
use diesel::{
    pg::{Pg, PgConnection},
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) {
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Unable to run migrations on database");
    log::info!("Ran migrations on database for startup");
}

pub fn get_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
    let conn = pool.get().context("Unable to get connection from pool");
    log::trace!("Got database connection from pool");

    conn
}

pub fn pool() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .context("Unable to establish connection pool");
    log::info!("Created connection pool");

    pool
}

pub fn init() -> Result<Pool<ConnectionManager<PgConnection>>> {
    let pool = pool()?;
    let conn = &mut get_connection(&pool)?;

    let migrate: bool = std::env::var("MIGRATE")
        .unwrap_or("false".to_owned())
        .parse()?;
    if migrate {
        run_migrations(conn);
        log::info!("Attempting to migrate")
    }

    Ok(pool)
}
