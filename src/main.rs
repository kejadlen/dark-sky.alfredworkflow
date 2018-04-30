#![feature(iterator_step_by, range_contains, try_from)]
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

mod dark_sky;
mod errors;
mod forecast;
mod geocode;
mod location;
mod precipitation;
mod sparkline;
mod theme;

use std::convert::TryFrom;
use std::env;

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
    let units = env::var("FORECAST_UNITS").unwrap_or_else(|_| "auto".into());
    let units = match units.as_str() {
        "auto" => forecast::Units::Auto,
        "ca" => forecast::Units::Ca,
        "uk2" => forecast::Units::Uk2,
        "us" => forecast::Units::Us,
        "si" => forecast::Units::Si,
        units => bail!("invalid `FORECAST_UNITS`: '{}'", units),
    };

    let dark_sky = dark_sky::DarkSky {
        dark_sky_api_key,
        location,
        theme,
        units,
    };

    dark_sky.run()
});

fn location() -> Result<location::Location> {
    let args: Vec<_> = env::args().skip(1).collect();
    let query = args.join(" ");
    let location = parse_default_location()?;
    match (query, location) {
        (ref query, _) if !query.is_empty() => {
            let api_key = env::var("GOOGLE_API_KEY")?;
            let geocoder = geocode::Geocoder::new(&api_key);
            geocoder.geocode(query)
        }
        (_, Some(ref location)) => Ok(location.clone()),
        _ => location::Location::from_ip(),
    }
}

fn parse_default_location() -> Result<Option<location::Location>> {
    let location = match env::var("DEFAULT_LAT_LONG") {
        Ok(ref lat_long) if !lat_long.is_empty() => {
            let coord = location::Coordinate::try_from(lat_long.as_str())
                .map_err(|_| Error::from(format!("invalid `DEFAULT_LAT_LONG`: {}", lat_long)))?;
            let description = env::var("DEFAULT_LOCATION").unwrap_or_else(|_| "".into());
            let location = location::Location { description, coord };
            Some(location)
        }
        _ => None,
    };
    Ok(location)
}
