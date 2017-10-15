use serde::Deserialize;

mod precipitation {
    use std::cmp::Ordering;
    use std::fmt;

    #[derive(Clone, Debug, Deserialize)]
    pub struct Intensity(pub f64);

    impl Intensity {
        pub fn humanized(&self) -> String {
            let intensity = if (0.0..=0.002).contains(self.0) {
                "no"
            } else if (0.002..=0.017).contains(self.0) {
                "very light"
            } else if (0.017..=0.1).contains(self.0) {
                "light"
            } else if (0.1..=0.4).contains(self.0) {
                "moderate"
            } else {
                "heavy"
            };
            intensity.into()
        }
    }

    impl Eq for Intensity {}
    impl PartialEq for Intensity {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl PartialOrd for Intensity {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Intensity {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl Eq for Probability {}
    impl PartialEq for Probability {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl PartialOrd for Probability {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl Ord for Probability {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl fmt::Display for Intensity {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let intensity = format!("{:.3}", (self.0 * 1000.).round() / 1000.);
            let mut intensity = String::from(intensity.trim_right_matches('0'));
            if intensity.ends_with('.') {
                intensity.push('0');
            }
            write!(f, "{}\"", intensity)
        }
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Probability(pub f64);

    impl fmt::Display for Probability {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}%", (self.0 * 100.).round())
        }
    }

    #[test]
    fn test_fmt() {
        let i = Intensity(0.);
        assert_eq!(format!("{}", i), "0.0\"");

        let i = Intensity(0.1);
        assert_eq!(format!("{}", i), "0.1\"");

        let i = Intensity(0.12);
        assert_eq!(format!("{}", i), "0.12\"");

        let i = Intensity(0.1234);
        assert_eq!(format!("{}", i), "0.123\"");

        let i = Intensity(0.1235);
        assert_eq!(format!("{}", i), "0.124\"");
    }
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
    #[serde(rename = "precipIntensity")] pub precip_intensity: Option<precipitation::Intensity>,
    #[serde(rename = "precipProbability")]
    pub precip_probability:
        Option<precipitation::Probability>,
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

    pub fn precip_intensities(&self) -> Vec<precipitation::Intensity> {
        self.data
            .iter()
            .flat_map(|x| x.precip_intensity.clone())
            .collect()
    }

    pub fn precip_probabilities(&self) -> Vec<precipitation::Probability> {
        self.data
            .iter()
            .flat_map(|x| x.precip_probability.clone())
            .collect()
    }
}
