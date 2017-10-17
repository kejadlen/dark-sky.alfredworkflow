use alphred::Item;
use chrono::prelude::*;
use reqwest;

use location;
use errors::*;
use forecast;
use sparkline;
use theme::Theme;

#[derive(Debug)]
pub struct DarkSky {
    pub dark_sky_api_key: String,
    pub location: location::Location,
    pub theme: Theme,
    pub units: forecast::Units,
}

impl DarkSky {
    pub fn run(&self) -> Result<()> {
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

        if let Some(daily) = forecast.daily {
            daily
                .data
                .iter()
                .take(5)
                .flat_map(|point| self.daily(point))
                .for_each(|x| items.push(x));
        }

        let json = json!({ "items": items });
        println!("{}", json);

        Ok(())
    }

    fn forecast(&self, coord: location::Coordinate) -> Result<forecast::Forecast> {
        let location::Coordinate(lat, long) = coord;
        let units = match self.units {
            forecast::Units::Auto => "auto",
            forecast::Units::Ca => "ca",
            forecast::Units::Uk2 => "uk2",
            forecast::Units::Us => "us",
            forecast::Units::Si => "si",
        };
        let url = format!(
            "https://api.darksky.net/forecast/{}/{},{}?units={}",
            self.dark_sky_api_key,
            lat,
            long,
            units,
        );
        Ok(reqwest::get(&url)?.json()?)
    }

    fn arg(&self) -> String {
        let location::Coordinate(lat, long) = self.location.coord;
        format!("{:.4},{:.4}", lat, long)
    }

    fn currently(&self, point: &forecast::Point) -> Option<Item> {
        if let (Some(title), Some(temp), Some(apparent_temp), Some(icon)) = (
            point.summary.clone(),
            point.temp,
            point.apparent_temperature.clone(),
            point.icon.clone(),
        ) {
            let mut subtitle = vec![
                format!("{}°", temp.round()),
                format!("Feels like {}", apparent_temp),
            ];
            if let Some(human_precip) = point.human_precipitation() {
                subtitle.push(human_precip);
            }
            let subtitle = subtitle.join(" · ");

            let mut item = Item::new(title);
            item = item.subtitle(&subtitle);
            item = item.arg(&self.arg());
            if let Some(path) = self.theme.icon_path(&icon) {
                item = item.icon(path.as_path());
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
                    5,
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
            if let Some(path) = self.theme.icon_path(&icon) {
                item = item.icon(path.as_path());
            }
            Some(item)
        } else {
            None
        }
    }

    fn daily(&self, point: &forecast::Point) -> Option<Item> {
        if let (Some(ref summary), Some(ref min), Some(ref max), Some(ref icon)) = (
            point.summary.clone(),
            point.apparent_temperature_min.clone(),
            point.apparent_temperature_max.clone(),
            point.icon.clone(),
        ) {
            let weekday = if point.time.date() == Local::today() {
                "Today".into()
            } else {
                point.time.format("%A").to_string()
            };
            let title = format!("{} - {}", weekday, summary);
            let subtitle = format!("Low: {} · High: {}", min, max);
            let arg = format!("{}/{}", self.arg(), point.time.timestamp());
            let mut item = Item::new(title);
            item = item.subtitle(&subtitle);
            item = item.arg(&arg);
            if let Some(path) = self.theme.icon_path(icon) {
                item = item.icon(path.as_path());
            }
            Some(item)
        } else {
            None
        }
    }
}
