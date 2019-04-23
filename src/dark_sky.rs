use alphred::Item;
use chrono::prelude::*;
use reqwest;

use errors::*;
use forecast;
use location;
use sparkline;
use theme::Theme;

#[derive(Debug)]
pub struct DarkSky {
    pub dark_sky_api_key: String,
    pub location: location::Location,
    pub theme: Theme,
    pub units: forecast::Units,
    pub lang: forecast::Lang,
}

impl DarkSky {
    pub fn run(&self) -> Result<()> {
        let forecast = self.forecast(&self.location.coord)?;

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

    fn forecast(&self, coord: &location::Coordinate) -> Result<forecast::Forecast> {
        let &location::Coordinate(lat, long) = coord;
        let units = match self.units {
            forecast::Units::Auto => "auto",
            forecast::Units::Ca => "ca",
            forecast::Units::Uk2 => "uk2",
            forecast::Units::Us => "us",
            forecast::Units::Si => "si",
        };
        let lang = match self.lang {
            forecast::Lang::Arabic => "ar",
            forecast::Lang::Azerbaijani => "az",
            forecast::Lang::Belarusian => "be",
            forecast::Lang::Bulgarian => "bg",
            forecast::Lang::Bengali => "bn",
            forecast::Lang::Bosnian => "bs",
            forecast::Lang::Catalan => "ca",
            forecast::Lang::Czech => "cs",
            forecast::Lang::Danish => "da",
            forecast::Lang::German => "de",
            forecast::Lang::Greek => "el",
            forecast::Lang::English => "en",
            forecast::Lang::Esperanto => "eo",
            forecast::Lang::Spanish => "es",
            forecast::Lang::Estonian => "et",
            forecast::Lang::Finnish => "fi",
            forecast::Lang::French => "fr",
            forecast::Lang::Hebrew => "he",
            forecast::Lang::Hindi => "hi",
            forecast::Lang::Croatian => "hr",
            forecast::Lang::Hungarian => "hu",
            forecast::Lang::Indonesian => "id",
            forecast::Lang::Icelandic => "is",
            forecast::Lang::Italian => "it",
            forecast::Lang::Japanese => "ja",
            forecast::Lang::Georgian => "ka",
            forecast::Lang::Kannada => "kn",
            forecast::Lang::Korean => "ko",
            forecast::Lang::Cornish => "kw",
            forecast::Lang::Latvian => "lv",
            forecast::Lang::Malayam => "ml",
            forecast::Lang::Marathi => "mr",
            forecast::Lang::NorwegianBokmal => "nb",
            forecast::Lang::Dutch => "nl",
            forecast::Lang::Punjabi => "pa",
            forecast::Lang::Polish => "pl",
            forecast::Lang::Portuguese => "pt",
            forecast::Lang::Romanian => "ro",
            forecast::Lang::Russian => "ru",
            forecast::Lang::Slovak => "sk",
            forecast::Lang::Slovenian => "sl",
            forecast::Lang::Serbian => "sr",
            forecast::Lang::Swedish => "sv",
            forecast::Lang::Tamil => "ta",
            forecast::Lang::Telugu => "te",
            forecast::Lang::Tetum => "tet",
            forecast::Lang::Turkish => "tr",
            forecast::Lang::Ukrainian => "uk",
            forecast::Lang::Urdu => "ur",
            forecast::Lang::IgpayAtinlay => "x-pig-latin",
            forecast::Lang::SimplifiedChinese => "zh",
            forecast::Lang::TraditionalChinese => "zh-tw",
        };
        let url = format!(
            "https://api.darksky.net/forecast/{}/{},{}?units={}&lang={}",
            self.dark_sky_api_key, lat, long, units, lang
        );
        Ok(reqwest::get(&url)?.json()?)
    }

    fn arg(&self) -> String {
        let location::Coordinate(lat, long) = self.location.coord;
        format!("{:.4},{:.4}", lat, long)
    }

    fn currently(&self, point: &forecast::Point) -> Option<Item> {
        let title = point.summary.clone()?;
        let temp = point.temp?;
        let apparent_temp = point.apparent_temperature.clone()?;
        let icon = point.icon.clone()?;

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
    }

    fn minutely(&self, block: &forecast::Block) -> Option<Item> {
        let title = block.summary.clone()?;
        let icon = block.icon.clone()?;

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
        if let (Some(min), Some(max)) = (probabilities.iter().min(), probabilities.iter().max()) {
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
    }

    fn daily(&self, point: &forecast::Point) -> Option<Item> {
        let summary = point.summary.clone()?;
        let min = point.apparent_temperature_min.clone()?;
        let max = point.apparent_temperature_max.clone()?;
        let icon = point.icon.clone()?;

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
        if let Some(path) = self.theme.icon_path(&icon) {
            item = item.icon(path.as_path());
        }
        Some(item)
    }
}
