use std::fmt;
use std::result;

use chrono::prelude::*;
use serde::Deserialize;

use precipitation::{Intensity, Probability};

#[derive(Clone, Debug, Deserialize)]
pub struct Temperature(f64);

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°", self.0.round())
    }
}

#[derive(Debug)]
pub enum Units {
    Auto,
    Ca,
    Uk2,
    Us,
    Si,
}

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
    fn deserialize<D>(deserializer: D) -> result::Result<Icon, D::Error>
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
    pub daily: Option<Block>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    #[serde(rename = "temperature")]
    pub temp: Option<f64>,
    pub apparent_temperature: Option<Temperature>,
    pub apparent_temperature_min: Option<Temperature>,
    pub apparent_temperature_max: Option<Temperature>,
    pub icon: Option<Icon>,
    pub precip_intensity: Option<Intensity>,
    pub precip_probability: Option<Probability>,
    pub summary: Option<String>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub time: DateTime<Local>,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> result::Result<DateTime<Local>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    let unix_time = i64::deserialize(deserializer)?;
    Ok(Local.timestamp(unix_time, 0))
}

impl Point {
    pub fn human_precipitation(&self) -> Option<String> {
        let intensity = self.precip_intensity.clone()?;
        let probability = self.precip_probability.clone()?;
        if probability.0 > 0. {
            Some(format!(
                "{} chance of {} rain.",
                probability,
                intensity.humanized()
            ))
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub data: Vec<Point>,
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
