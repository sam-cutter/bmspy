use product::Product;

mod product;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to pool.");
    
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

}
