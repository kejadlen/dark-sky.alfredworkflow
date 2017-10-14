#![feature(inclusive_range_syntax, range_contains)]
#![recursion_limit = "1024"]

extern crate alphred;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod coordinate;
mod forecast;
mod errors;

use std::env;

use alphred::Item;

use coordinate::Coordinate;
use forecast::{Forecast, Icon};
use errors::*;

quick_main!(|| DarkSky::new()?.run());

struct DarkSky {
    dark_sky_api_key: String,
}

impl DarkSky {
    fn new() -> Result<Self> {
        let dark_sky_api_key = env::var("DARK_SKY_API_KEY")?;
        Ok(Self { dark_sky_api_key })
    }

    fn run(&self) -> Result<()> {
        let mut items = Vec::new();

        let location = self.location()?;
        let Coordinate(lat, long) = location.coord;
        let arg = format!("{:.4},{:.4}", lat, long);

        let item = Item::new(location.description)
            .subtitle("• Powered by Dark Sky")
            .arg(&arg)
            .icon("icons/dark_sky.png");
        items.push(item);

        let forecast = self.forecast(location.coord)?;

        let currently = forecast.currently;
        let title = currently.summary.clone();
        let mut subtitle = vec![
            format!("{}°", currently.temp.round()),
            format!("Feels like {}°", currently.apparent_temp.round()),
        ];
        if let Some(precip) = currently.precipitation().map(|p| format!("{}", p)) {
            subtitle.push(precip)
        }
        let subtitle = subtitle.join(" · ");
        let mut item = Item::new(title);
        item = item.subtitle(&subtitle);
        item = item.arg(&arg);
        if let Some(path) = Self::translate_icon(&currently.icon) {
            item = item.icon(format!("Dark-{}", path).as_str());
        }
        items.push(item);

        let json = json!({
            "items": items
        });
        println!("{}", json);

        Ok(())
    }

    fn location(&self) -> Result<Location> {
        let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
        let description = format!("{}, {}", ip_info.city, ip_info.region);
        let coord = ip_info.coord;
        Ok(Location { description, coord })
    }

    fn forecast(&self, coord: Coordinate) -> Result<Forecast> {
        let Coordinate(lat, long) = coord;
        let url = format!(
            "https://api.darksky.net/forecast/{}/{},{}",
            self.dark_sky_api_key,
            lat,
            long
        );
        Ok(reqwest::get(&url)?.json()?)
    }

    fn translate_icon(icon: &forecast::Icon) -> Option<String> {
        match *icon {
            Icon::ClearDay => Some("Sun"),
            Icon::ClearNight => Some("Moon"),
            Icon::Rain => Some("Cloud-Rain"),
            Icon::Snow => Some("Cloud-Snow"),
            Icon::Sleet => Some("Cloud-Snow-Alt"),
            Icon::Wind => Some("Wind"),
            Icon::Fog => Some("Cloud-Fog"),
            Icon::Cloudy => Some("Cloud"),
            Icon::PartlyCloudyDay => Some("Cloud-Sun"),
            Icon::PartlyCloudyNight => Some("Cloud-Moon"),
            Icon::Unknown(_) => None,
        }.map(String::from)
    }
}

#[derive(Debug)]
struct Location {
    description: String,
    coord: Coordinate,
}

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc")]
    coord: Coordinate,
    city: String,
    region: String,
}
