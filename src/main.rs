#![feature(inclusive_range_syntax, iterator_step_by, range_contains)]
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
mod sparkline;

use std::env;

use alphred::Item;

use coordinate::Coordinate;
use forecast::{Forecast, Icon};
use errors::*;

quick_main!(|| DarkSky::new()?.run());

struct DarkSky {
    dark_sky_api_key: String,
    location: Location,
}

impl DarkSky {
    fn new() -> Result<Self> {
        let dark_sky_api_key = env::var("DARK_SKY_API_KEY")?;
        let location = Self::location()?;

        Ok(Self {
            dark_sky_api_key,
            location,
        })
    }

    fn location() -> Result<Location> {
        let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
        let description = format!("{}, {}", ip_info.city, ip_info.region);
        let coord = ip_info.coord;
        Ok(Location { description, coord })
    }

    fn run(&self) -> Result<()> {
        let forecast = self.forecast(self.location.coord.clone())?;

        let mut items = Vec::new();

        let item = Item::new(self.location.description.clone())
            .subtitle("• Powered by Dark Sky")
            .arg(&self.arg())
            .icon("icons/dark_sky.png");
        items.push(item);

        if let Some(item) = forecast.currently.and_then(|point| self.currently(&point)) {
            items.push(item);
        }

        if let Some(item) = forecast.minutely.and_then(|block| self.minutely(&block)) {
            items.push(item);
        }

        let json = json!({ "items": items });
        println!("{}", json);

        Ok(())
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

    fn arg(&self) -> String {
        let Coordinate(lat, long) = self.location.coord;
        format!("{:.4},{:.4}", lat, long)
    }

    fn currently(&self, point: &forecast::Point) -> Option<Item> {
        if let (Some(title), Some(temp), Some(apparent_temp), Some(icon)) = (
            point.summary.clone(),
            point.temp,
            point.apparent_temp,
            point.icon.clone(),
        ) {
            let mut subtitle = vec![
                format!("{}°", temp.round()),
                format!("Feels like {}°", apparent_temp.round()),
            ];
            if let (Some(intensity), Some(probability)) = (
                point.precip_intensity.clone(),
                point.precip_probability.clone(),
            ) {
                if intensity.0 > 0. {
                    subtitle.push(format!(
                        "{} chance of {} rain.",
                        probability,
                        intensity.humanized()
                    ));
                }
            }
            let subtitle = subtitle.join(" · ");

            let mut item = Item::new(title);
            item = item.subtitle(&subtitle);
            item = item.arg(&self.arg());
            if let Some(path) = Self::translate_icon(&icon) {
                item = item.icon(format!("Dark-{}", path).as_str());
            }
            Some(item)
        } else {
            None
        }
    }

    fn minutely(&self, block: &forecast::Block) -> Option<Item> {
        if let (Some(title), Some(icon)) = (block.summary.clone(), block.icon.clone()) {
            let mut item = Item::new(title);

            let mut subtitle = Vec::new();
            let intensities = block.precip_intensities();
            if let (Some(min), Some(max)) = (intensities.iter().min(), intensities.iter().max()) {
                let sparkline = sparkline::Ascii::new(
                    min.0,
                    max.0,
                    intensities.clone().iter().map(|x| x.0).collect(),
                    4,
                );
                subtitle.push(format!("{} {} {}", min, sparkline, max));
            }
            let probabilities = block.precip_probabilities();
            if let (Some(min), Some(max)) = (probabilities.iter().min(), probabilities.iter().max())
            {
                let sparkline = sparkline::Ascii::new(
                    min.0,
                    1.,
                    probabilities.clone().iter().map(|x| x.0).collect(),
                    5,
                );
                subtitle.push(format!("{} {} {}", min, sparkline, max));
            }
            let subtitle = subtitle.join(" · ");

            item = item.subtitle(&subtitle);
            item = item.arg(&self.arg());
            if let Some(path) = Self::translate_icon(&icon) {
                item = item.icon(format!("Dark-{}", path).as_str());
            }
            Some(item)
        } else {
            None
        }
    }

    fn translate_icon(icon: &Icon) -> Option<String> {
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
    #[serde(rename = "loc")] coord: Coordinate,
    city: String,
    region: String,
}
