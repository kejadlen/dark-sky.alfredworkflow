use std::fmt;
use serde::Deserialize;

#[derive(Debug)]
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
    pub currently: Point,
}

#[derive(Debug, Deserialize)]
pub struct Point {
    #[serde(rename = "temperature")]
    pub temp: f64,
    #[serde(rename = "apparentTemperature")]
    pub apparent_temp: f64,
    pub icon: Icon,
    #[serde(rename = "precipIntensity")]
    precip_intensity: Option<f64>,
    #[serde(rename = "precipProbability")]
    precip_probability: Option<f64>,
    pub summary: String,
}

impl Point {
    pub fn precipitation(&self) -> Option<Precipitation> {
        self.precip_intensity.and_then(|intensity| {
            if intensity == 0.0 {
                return None
            }

            self.precip_probability.map(|probability| {
                Precipitation {
                    intensity,
                    probability,
                }
            })
        })
    }
}

pub struct Precipitation {
    intensity: f64,
    probability: f64,
}

impl Precipitation {
    fn human_intensity(&self) -> String {
        let intensity = if (0.0..=0.002).contains(self.intensity) {
            "no"
        } else if (0.002..=0.017).contains(self.intensity) {
            "very light"
        } else if (0.017..=0.1).contains(self.intensity) {
            "light"
        } else if (0.1..=0.4).contains(self.intensity) {
            "moderate"
        } else {
            "heavy"
        };
        intensity.into()
    }
}

impl fmt::Display for Precipitation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}% chance of {} rain.",
            (self.probability * 100.0).round(),
            self.human_intensity()
        )
    }
}
