pub struct Product {
    pub back_market_uuid: String,
    pub title: String,
}

enum BackMarketUUIDExtractionError {
    InvalidURL,
    InvalidHostname,
    InvalidPattern,
    InvalidUUID,
}

impl BackMarketUUIDExtractionError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidURL => "The URL is not valid. Make sure that the entire URL is copied and pasted.",
            Self::InvalidHostname => "The URL does not seem to be a Back Market URL. Make sure that the URL is from Back Market.",
            Self::InvalidPattern => "The URL does not seem to be a valid Back Market product URL. Make sure that the URL is copied from a product page.",
            Self::InvalidUUID => "The URL does not seem to contain a valid Back Market product id. Make sure that the entire URL is copied.",
        }
    }
}

impl Product {
    pub fn from_url(url: &str) -> Result<Self, String> {
        let back_market_uuid = match Self::extract_back_market_uuid_from_url(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e.message().to_string()),
        };

        // TODO: Fetch the product title
        let title = String::new();

        Result::Ok(Product {
            back_market_uuid,
            title,
        })
    }

    fn extract_back_market_uuid_from_url(
        url: &str,
    ) -> Result<String, BackMarketUUIDExtractionError> {
        // Ensure that the URL is valid
        let url = match url::Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(BackMarketUUIDExtractionError::InvalidURL),
        };

        // Ensure that the URL contains a hostname
        let host = match url.host() {
            Some(host) => host.to_string(),
            None => return Err(BackMarketUUIDExtractionError::InvalidHostname),
        };

        // Accepted Back Market hostnames
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

        // If the hostname is not a valid Back Market hostname, return an error
        if !VALID_HOSTNAMES.contains(&host.as_str()) {
            return Err(BackMarketUUIDExtractionError::InvalidHostname);
        }

        // Collect the URL path segments
        let url_path_segments: Vec<&str> = match url.path_segments() {
            Some(segments) => segments.collect(),
            None => return Err(BackMarketUUIDExtractionError::InvalidPattern),
        };

        // Ensure that the URL matches the pattern of /{locale}/p/{slug}/{uuid}
        if url_path_segments.len() != 4 || url_path_segments[1] != "p" {
            return Err(BackMarketUUIDExtractionError::InvalidPattern);
        }

        let back_market_uuid = url_path_segments[3];

        // Regex of a UUID
        let uuid_regex = regex::Regex::new(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
        )
        .unwrap();

        // Ensure that the UUID matches the UUID regex
        match uuid_regex.is_match(back_market_uuid) {
            true => Ok(back_market_uuid.to_string()),
            false => Err(BackMarketUUIDExtractionError::InvalidUUID),
        }
    }
}

pub struct Snapshot {
    pub product: Product,
    pub grade: Grade,
    pub price: Option<f32>,
}

pub enum Grade {
    Fair,
    Good,
    Excellent,
}
