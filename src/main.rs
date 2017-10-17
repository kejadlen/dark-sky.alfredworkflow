#![feature(inclusive_range_syntax, iterator_step_by, range_contains, try_from)]
#![recursion_limit = "1024"]

extern crate alphred;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate url;

mod coordinate;
mod dark_sky;
mod errors;
mod forecast;
mod precipitation;
mod sparkline;
mod theme;

use std::env;

use coordinate::Coordinate;
use errors::*;
use theme::Theme;

quick_main!(|| {
    let dark_sky_api_key = env::var("DARK_SKY_API_KEY")?;
    let location = location()?;
    let theme = if env::var("LIGHT_ICONS") == Ok("true".into()) {
        Theme::Light
    } else {
        Theme::Dark
    };
    let units = env::var("FORECAST_UNITS")
        .map(|units| match units.as_str() {
            "ca" => forecast::Units::Ca,
            "uk2" => forecast::Units::Uk2,
            "us" => forecast::Units::Us,
            "si" => forecast::Units::Si,
            _ => forecast::Units::Auto,
        })
        .unwrap_or_else(|_| forecast::Units::Auto);

    let dark_sky = dark_sky::DarkSky {
        dark_sky_api_key,
        location,
        theme,
        units,
    };

    dark_sky.run()
});

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc")] coord: Coordinate,
    city: String,
    region: String,
}

fn location() -> Result<dark_sky::Location> {
    let args: Vec<_> = env::args().skip(1).collect();
    let query = args.join(" ");
    if query.is_empty() {
        let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
        let description = format!("{}, {}", ip_info.city, ip_info.region);
        let coord = ip_info.coord;
        Ok(dark_sky::Location { description, coord })
    } else {
        geocode(&query)
    }
}

mod geocode {
    use coordinate::Coordinate;
    use dark_sky;

    #[derive(Debug, Deserialize)]
    pub struct Response {
        results: Vec<Result>,
    }

    impl Response {
        pub fn location(&self) -> Option<dark_sky::Location> {
            let result = self.results.first();
            result.map(|result| {
                let description = result.formatted_address.clone();
                let location = &result.geometry.location;
                let coord = Coordinate(location.lat, location.lng);
                dark_sky::Location { description, coord }
            })
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
}

fn geocode(address: &str) -> Result<dark_sky::Location> {
    let client = reqwest::Client::new();

    let api_key = env::var("GOOGLE_API_KEY")?;
    let mut url = url::Url::parse("https://maps.googleapis.com/maps/api/geocode/json")?;
    url.query_pairs_mut().append_pair("address", address);
    url.query_pairs_mut().append_pair("api_key", &api_key);
    let response: geocode::Response = client.get(url).send()?.json()?;

    response
        .location()
        .ok_or_else(|| format!("unable to geocode address '{}'", address).into())
}
