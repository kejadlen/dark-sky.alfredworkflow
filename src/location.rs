use std::result;

use reqwest;
use serde::Deserialize;

use errors::*;

#[derive(Clone, Debug)]
pub struct Location {
    pub description: String,
    pub coord: Coordinate,
}

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc")] coord: Coordinate,
    city: String,
    region: String,
}

impl Location {
    pub fn from_ip() -> Result<Self> {
        let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json()?;
        let description = format!("{}, {}", ip_info.city, ip_info.region);
        let coord = ip_info.coord;
        Ok(Self { description, coord })
    }
}

#[derive(Clone, Debug)]
pub struct Coordinate(pub f64, pub f64);

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> result::Result<Coordinate, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut split = s.split(',');
        let lat = split
            .next()
            .and_then(|lat| lat.parse().ok())
            .ok_or_else(|| ::serde::de::Error::custom(""))?;
        let long = split
            .next()
            .and_then(|long| long.parse().ok())
            .ok_or_else(|| ::serde::de::Error::custom(""))?;
        Ok(Coordinate(lat, long))
    }
}

#[test]
fn test_deserializing_coordinate() {
    let c: result::Result<Coordinate, _> = ::serde_json::from_str("\"\"");
    assert!(c.is_err());

    let c: result::Result<Coordinate, _> = ::serde_json::from_str("\"123\"");
    assert!(c.is_err());

    let c: result::Result<Coordinate, _> = ::serde_json::from_str("\"123,\"");
    assert!(c.is_err());

    let Coordinate(lat, long) = ::serde_json::from_str("\"123,-123\"").unwrap();
    assert_eq!(lat, 123.0);
    assert_eq!(long, -123.0);
}
