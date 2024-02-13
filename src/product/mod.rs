pub struct Product {
    pub back_market_uuid: String,
    pub title: String,
}

impl Product {
    pub fn new(url: &str) -> Result<Self, &str> {
        let back_market_uuid = match extract_back_market_uuid(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e),
        };

        // TODO: Fetch the product title
        let title = String::new();

        Result::Ok(Product {
            back_market_uuid,
            title,
        })
    }
}

pub fn extract_back_market_uuid(url: &str) -> Result<String, &str> {
    // TODO: Standardise error messages
    // TODO: Move to a separate module

    // Ensure that the URL is valid
    let url = match url::Url::parse(url) {
        Ok(url) => url,
        Err(_) => return Err("Unable to interpret product URL."),
    };

    // Ensure that the URL contains a hostname
    let host = match url.host() {
        Some(host) => host.to_string(),
        None => return Err("Invalid product URL hostname."),
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
        return Err("Invalid product URL hostname.");
    }

    // Collect the URL path segments
    let url_path_segments: Vec<&str> = match url.path_segments() {
        Some(segments) => segments.collect(),
        None => return Err("Invalid product URL path."),
    };

    // Ensure that the URL matches the pattern of /{locale}/p/{slug}/{uuid}
    if url_path_segments.len() != 4 || url_path_segments[1] != "p" {
        return Err("Invalid product URL pattern.");
    }

    let back_market_uuid = url_path_segments[3];

    // Regex of a UUID
    let uuid_regex = regex::Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
    )
    .unwrap();

    // Ensure that the UUID matches the UUID regex
    match uuid_regex.is_match(back_market_uuid) {
        true => return Ok(back_market_uuid.to_string()),
        false => return Err("Invalid product UUID."),
    }
}

pub struct Snapshot {
    pub product: Product,
    pub grade: Grade,
    pub availability: Availability,
}

pub enum Availability {
    Available(f64),
    NotAvailable,
}

pub enum Grade {
    Fair,
    Good,
    Excellent,
}
