#![feature(inclusive_range_syntax, iterator_step_by, range_contains)]
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

mod coordinate;
mod dark_sky;
mod errors;
mod forecast;
mod precipitation;
mod sparkline;
mod theme;

use std::env;

use coordinate::Coordinate;
use dark_sky::{DarkSky, Location};
use errors::*;
use theme::Theme;

quick_main!(|| {
    let dark_sky_api_key = env::var("DARK_SKY_API_KEY")?;
    let theme = if env::var("LIGHT_ICONS") == Ok("true".into()) {
        Theme::Light
    } else {
        Theme::Dark
    };
    let location = location()?;

    let dark_sky = DarkSky{
        dark_sky_api_key,
        theme,
        location,
    };

    dark_sky.run()
});

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc")] coord: Coordinate,
    city: String,
    region: String,
}

fn location() -> Result<Location> {
    let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
    let description = format!("{}, {}", ip_info.city, ip_info.region);
    let coord = ip_info.coord;
    Ok(Location { description, coord })
}
