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

fn extract_back_market_uuid(url: &str) -> Result<String, &str> {
    // TODO: Standardise error messages
    // TODO: Move to a separate module

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

    let url = match url::Url::parse(url) {
        Ok(url) => url,
        Err(_) => {
            return Err("Unable to interpret product URL.");
        }
    };

    if !VALID_HOSTNAMES.contains(&url.host_str().unwrap_or("")) {
        return Err("Invalid product URL hostname.");
    }

    let url_path_segments: Vec<&str> = match url.path_segments() {
        Some(segments) => segments.filter(|segment| !segment.is_empty()).collect(),
        None => return Err("Invalid product URL path."),
    };

    if url_path_segments.len() != 4 || url_path_segments[1] != "p" {
        return Err("Invalid product URL pattern.");
    }

    let back_market_uuid = url_path_segments[3];

    let uuid_regex = regex::Regex::new(
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
    )
    .unwrap();

    if !uuid_regex.is_match(back_market_uuid) {
        return Err("Invalid product UUID.");
    }

    Ok(back_market_uuid.to_string())
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
