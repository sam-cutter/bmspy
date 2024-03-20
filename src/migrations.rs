use sqlx::migrate;

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), migrate::MigrateError> {
    match migrate!("./migrations").run(pool).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
