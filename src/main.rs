mod client;
mod migrations;
mod product;
mod snapshot;

use product::Product;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPool::connect(&database_url).await {
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

    test_product(&pool).await;
}

async fn test_product(pool: &PgPool) {
    let product = match Product::from_url("https://www.backmarket.co.uk/en-gb/p/iphone-13-128-gb-unlocked/ef5660d2-6883-4b81-b47d-86e5720687ef#l=12&scroll=false").await {
        Ok(product) => product,
        Err(e) => {
            return;
        }
    };

    product.track(pool).await.expect("Failed to track product");
}
