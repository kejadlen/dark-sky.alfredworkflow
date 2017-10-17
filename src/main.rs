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
mod geocode;
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

    let location = env::var("DEFAULT_LAT_LONG").ok().and_then(|lat_long| {
        let mut split = lat_long.split(',');
        split
            .next()
            .and_then(|lat| lat.parse::<f64>().ok())
            .and_then(|lat| {
                split
                    .next()
                    .and_then(|long| long.parse::<f64>().ok())
                    .map(|long| Coordinate(lat, long))
            })
            .map(|coord| {
                let description = env::var("DEFAULT_LOCATION").unwrap_or("".into());
                dark_sky::Location { description, coord }
            })
    });

    match (query, location) {
        (ref query, _) if !query.is_empty() => {
            let api_key = env::var("GOOGLE_API_KEY")?;
            let geocoder = geocode::Geocoder::new(&api_key);
            geocoder.geocode(&query)
        }
        (_, Some(ref location)) => Ok(location.clone()),
        _ => {
            let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
            let description = format!("{}, {}", ip_info.city, ip_info.region);
            let coord = ip_info.coord;
            Ok(dark_sky::Location { description, coord })
        }
    }
}
