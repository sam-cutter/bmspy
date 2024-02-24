use rand::Rng;
use serde_derive::Deserialize;

const USER_AGENTS: [&str; 7] = [
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 13_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15"
];

#[derive(Deserialize)]
struct ApiResponse {
    title: String,
}

fn random_user_agent() -> &'static str {
    let index = rand::thread_rng().gen::<usize>() % USER_AGENTS.len();
    USER_AGENTS[index]
}

pub enum TitleFetchingError {
    ClientCreationFailure,
    RequestFailure,
    ProductNotFound,
    Other,
}

impl TitleFetchingError {
    pub fn message(&self) -> &str {
        match self {
            Self::ClientCreationFailure => "Something went wrong on our end. Try again later or contact support if the problem persists.",
            Self::RequestFailure => "Something went wrong on our end. It looks like Back Market isn't talking to us. Try again later or contact support if the problem persists.",
            Self::ProductNotFound => "We couldn't find the product you're looking for. The product URL might not actually exist. Make sure you copy it correctly, yourself.",
            Self::Other => "Something went wrong. It might be an issue with Back Market. Try again later or contact support if the problem persists.",
        }
    }
}

pub async fn fetch_title(back_market_uuid: &str) -> Result<String, TitleFetchingError> {
    let api_url = format!(
        "https://www.backmarket.co.uk/bm/product/{back_market_uuid}/technical_specifications",
    );

    let client = match reqwest::Client::builder()
        .user_agent(random_user_agent())
        .build()
    {
        Ok(client) => client,
        Err(_) => return Err(TitleFetchingError::ClientCreationFailure),
    };

    let response = match client
        .get(&api_url)
        .header("Accept-Language", "en-gb")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(TitleFetchingError::RequestFailure),
    };

    match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::NOT_FOUND => return Err(TitleFetchingError::ProductNotFound),
        _ => return Err(TitleFetchingError::RequestFailure),
    }

    let response: ApiResponse = match response.json().await {
        Ok(json) => json,
        Err(_) => return Err(TitleFetchingError::Other),
    };

    let title = response.title;

    Result::Ok(title)
}
