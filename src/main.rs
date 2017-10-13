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

use alphred::{Item, Icon};

use coordinate::Coordinate;
use forecast::Forecast;
use errors::*;

quick_main!(|| { DarkSky::new().run() });

struct DarkSky {}

impl DarkSky {
    fn new() -> Self {
        Self{}
    }

    fn run(&self) -> Result<()> {
        let mut items = Vec::new();

        let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
        let coord = ip_info.coord;
        let lat = coord.lat;
        let long = coord.long;
        let arg = format!("{:.4},{:.4}", lat, long);

        let api_key = env::var("DARK_SKY_API_KEY")?;
        let url = format!(
            "https://api.darksky.net/forecast/{}/{},{}",
            api_key,
            lat,
            long
        );
        let forecast: Forecast = reqwest::get(&url)?.json()?;

        let title = format!("{}, {}", ip_info.city, ip_info.region);
        let icon = Icon { path: "icons/dark_sky.png".into() };
        let item = Item::new(title)
            .subtitle("• Powered by Dark Sky")
            .arg(&arg)
            .icon(icon);
        items.push(item);

        let title = forecast.currently.summary;
        let subtitle = vec![
            format!("{}°", forecast.currently.temp.round()),
            format!("Feels like {}°", forecast.currently.apparent_temp.round()),
        ];
        let subtitle = subtitle.join(" · ");
        let mut item = Item::new(title);
        item = item.subtitle(&subtitle);
        item = item.arg(&arg);
        if let Some(path) = Self::translate_icon(&forecast.currently.icon) {
            let path = format!("Dark-{}", path).into();
            item = item.icon(Icon { path });
        }
        items.push(item);

        let json = json!({
            "items": items
        });
        println!("{}", json);

        Ok(())
    }

    fn translate_icon(icon: &forecast::Icon) -> Option<String> {
        match *icon {
            forecast::Icon::ClearDay => Some("Sun"),
            forecast::Icon::ClearNight => Some("Moon"),
            forecast::Icon::Rain => Some("Cloud-Rain"),
            forecast::Icon::Snow => Some("Cloud-Snow"),
            forecast::Icon::Sleet => Some("Cloud-Snow-Alt"),
            forecast::Icon::Wind => Some("Wind"),
            forecast::Icon::Fog => Some("Cloud-Fog"),
            forecast::Icon::Cloudy => Some("Cloud"),
            forecast::Icon::PartlyCloudyDay => Some("Cloud-Sun"),
            forecast::Icon::PartlyCloudyNight => Some("Cloud-Moon"),
            forecast::Icon::Unknown(_) => None,
        }.map(String::from)
    }
}

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc")]
    coord: Coordinate,
    city: String,
    region: String,
}
