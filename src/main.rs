mod migrations;
mod product;
mod snapshot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match sqlx::PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return;
        }
    };

    match migrations::run_migrations(&pool).await {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => {
            eprintln!("Failed to run migrations: {}", e);
            return;
        }
    }
}
