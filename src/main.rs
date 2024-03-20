mod product;
mod snapshot;

use product::Product;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to pool.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let product = Product::from_url("https://www.backmarket.co.uk/en-gb/p/macbook-pro-2020-13-inch-with-m1-8-core-and-8-core-gpu-8gb-ram-ssd-2tb-qwerty-english-uk/8948b82c-f746-4be0-a8b0-0758b1dc4acc#l=12").await;

    match product {
        Ok(product) => {
            // Print product details
            println!("Product UUID: {}", product.uuid());
            println!("Product Title: {}", product.title());
        }
        Err(e) => {
            eprintln!("Failed to create product: {:?}", e.message());
        }
    }
}
