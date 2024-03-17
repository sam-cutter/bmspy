use product::Product;

mod product;

#[tokio::main]
async fn main() {
    let url = "https://www.backmarket.co.uk/en-gb/p/macbook-air-2020-13-inch-with-m1-8-core-and-7-core-gpu-8gb-ram-ssd-2tb-qwerty-english-us/b5ebc79d-0304-41a6-b1ae-d2a487afa11f#l=12";
    match Product::from_url(url).await {
        Ok(product) => {
            println!("Back Market UUID: {}", product.back_market_uuid());
            println!("Title: {}", product.title());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
