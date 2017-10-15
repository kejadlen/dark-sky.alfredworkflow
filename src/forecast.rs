use serde::Deserialize;

use precipitation::{Intensity, Probability};

#[derive(Clone, Debug)]
pub enum Icon {
    ClearDay,
    ClearNight,
    Rain,
    Snow,
    Sleet,
    Wind,
    Fog,
    Cloudy,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Unknown(String),
}

impl<'de> Deserialize<'de> for Icon {
    fn deserialize<D>(deserializer: D) -> Result<Icon, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "clear-day" => Icon::ClearDay,
            "clear-night" => Icon::ClearNight,
            "rain" => Icon::Rain,
            "snow" => Icon::Snow,
            "sleet" => Icon::Sleet,
            "wind" => Icon::Wind,
            "fog" => Icon::Fog,
            "cloudy" => Icon::Cloudy,
            "partly-cloudy-day" => Icon::PartlyCloudyDay,
            "partly-cloudy-night" => Icon::PartlyCloudyNight,
            s => Icon::Unknown(s.into()),
        })
    }
}


#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub currently: Option<Point>,
    pub minutely: Option<Block>,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    #[serde(rename = "temperature")] pub temp: Option<f64>,
    #[serde(rename = "apparentTemperature")] pub apparent_temp: Option<f64>,
    pub icon: Option<Icon>,
    #[serde(rename = "precipIntensity")] pub precip_intensity: Option<Intensity>,
    #[serde(rename = "precipProbability")]
    pub precip_probability:
        Option<Probability>,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    data: Vec<Point>,
    pub summary: Option<String>,
    pub icon: Option<Icon>,
}

impl Block {
    // These aren't quite right since I'm just dropping precipitation values that aren't there...

    pub fn precip_intensities(&self) -> Vec<Intensity> {
        self.data
            .iter()
            .flat_map(|x| x.precip_intensity.clone())
            .collect()
    }

    pub fn precip_probabilities(&self) -> Vec<Probability> {
        self.data
            .iter()
            .flat_map(|x| x.precip_probability.clone())
            .collect()
    }
}
