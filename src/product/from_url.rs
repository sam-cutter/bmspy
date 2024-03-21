use super::Product;
use serde_derive::Deserialize;

impl Product {
    pub async fn from_url(url: &str) -> Result<Self, ProductCreationFromURLError> {
        let uuid = match extract_uuid_from_url(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e),
        };

        let title = match fetch_title(uuid.as_str()).await {
            Ok(title) => title,
            Err(e) => return Err(e),
        };

        Result::Ok(Product { uuid, title })
    }
}
pub enum ProductCreationFromURLError {
    InvalidURL,
    TitleFetchingError,
}

impl ProductCreationFromURLError {
    pub fn message(&self) -> &str {
        match self {
            ProductCreationFromURLError::InvalidURL => "It looks like the link you provided might not be valid. Make sure to copy the entire URL from your browser's address bar and try again.",
            ProductCreationFromURLError::TitleFetchingError => "Sorry - something's gone wrong on our end. Please try again later and, if the problem persists, contact support.",
        }
    }
}

const VALID_HOSTNAMES: [&str; 24] = [
    "www.backmarket.co.uk",
    "backmarket.co.uk",
    "www.backmarket.com",
    "backmarket.com",
    "www.backmarket.fr",
    "backmarket.fr",
    "www.backmarket.de",
    "backmarket.de",
    "www.backmarket.es",
    "backmarket.es",
    "www.backmarket.it",
    "backmarket.it",
    "www.backmarket.be",
    "backmarket.be",
    "www.backmarket.nl",
    "backmarket.nl",
    "www.backmarket.at",
    "backmarket.at",
    "www.backmarket.pt",
    "backmarket.pt",
    "www.backmarket.se",
    "backmarket.se",
    "www.backmarket.fi",
    "backmarket.fi",
];

fn extract_uuid_from_url(url: &str) -> Result<String, ProductCreationFromURLError> {
    // Ensure that the URL is valid
    let url = match url::Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err(ProductCreationFromURLError::InvalidURL),
    };

    // Ensure that the URL contains a hostname
    let host = match url.host() {
        Some(host) => host.to_string(),
        None => return Err(ProductCreationFromURLError::InvalidURL),
    };

    // If the hostname is not a valid Back Market hostname, return an error
    if !VALID_HOSTNAMES.contains(&host.as_str()) {
        return Err(ProductCreationFromURLError::InvalidURL);
    }

    // Collect the URL path segments
    let url_path_segments: Vec<&str> = match url.path_segments() {
        Some(segments) => segments.collect(),
        None => return Err(ProductCreationFromURLError::InvalidURL),
    };

    // Ensure that the URL matches the pattern of /{locale}/p/{slug}/{uuid}
    if url_path_segments.len() != 4 || url_path_segments[1] != "p" {
        return Err(ProductCreationFromURLError::InvalidURL);
    }

    let uuid = url_path_segments[3];

    // Regex of a UUID
    let uuid_regex = regex::Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
    )
    .unwrap();

    // Ensure that the UUID matches the UUID regex
    match uuid_regex.is_match(uuid) {
        true => Ok(uuid.to_string()),
        false => Err(ProductCreationFromURLError::InvalidURL),
    }
}

#[derive(Deserialize)]
struct TitleFetchAPIResponse {
    title: String,
}

async fn fetch_title(uuid: &str) -> Result<String, ProductCreationFromURLError> {
    // Format the API URL
    let api_url =
        format!("https://www.backmarket.co.uk/bm/product/{uuid}/technical_specifications",);

    let client = match crate::client::client() {
        Ok(client) => client,
        Err(_) => return Err(ProductCreationFromURLError::TitleFetchingError),
    };

    // Make the API request to fetch the product title
    let response = match client
        .get(&api_url)
        .header("Accept-Language", "en-gb")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err(ProductCreationFromURLError::TitleFetchingError),
    };

    // Check if the API request was successful
    match response.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::NOT_FOUND => return Err(ProductCreationFromURLError::InvalidURL),
        _ => return Err(ProductCreationFromURLError::TitleFetchingError),
    }

    // Map API response to a struct in order to extract the title
    let response: TitleFetchAPIResponse = match response.json().await {
        Ok(json) => json,
        Err(_) => return Err(ProductCreationFromURLError::TitleFetchingError),
    };

    let title = response.title;

    Result::Ok(title)
}
