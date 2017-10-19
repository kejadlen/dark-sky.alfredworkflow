use reqwest;
use url;

use errors;
use location;

pub struct Geocoder {
    api_key: String,
}

impl Geocoder {
    pub fn new(api_key: &str) -> Self {
        let api_key = api_key.into();
        Self { api_key }
    }

    pub fn geocode(&self, address: &str) -> errors::Result<location::Location> {
        let client = reqwest::Client::new();

        let mut url = url::Url::parse("https://maps.googleapis.com/maps/api/geocode/json")?;
        url.query_pairs_mut().append_pair("address", address);
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        let response: Response = client.get(url).send()?.json()?;

        response
            .location()
            .ok_or_else(|| format!("unable to geocode address '{}'", address).into())
    }
}

#[derive(Debug, Deserialize)]
struct Response {
    results: Vec<Result>,
}

impl Response {
    fn location(&self) -> Option<location::Location> {
        let result = self.results.first()?;
        let description = result.formatted_address.clone();
        let location = &result.geometry.location;
        let coord = location::Coordinate(location.lat, location.lng);
        Some(location::Location { description, coord })
    }
}

#[derive(Debug, Deserialize)]
struct Result {
    formatted_address: String,
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}
