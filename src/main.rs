#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;

use std::result;

use serde::Deserialize;

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json().map_err(
        Error::from,
    )?;
    println!("{:?}", ip_info);
    Ok(())
}

#[derive(Debug)]
struct Coordinate {
    lat: f64,
    long: f64,
}

#[derive(Debug, Deserialize)]
struct IPInfo {
    #[serde(rename = "loc", deserialize_with = "coordinate_from_string")]
    location: Coordinate,
    city: String,
    region: String,
}

fn coordinate_from_string<'de, D>(deserializer: D) -> result::Result<Coordinate, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut split = s.split(',');
    let lat = split.next().and_then(|lat| lat.parse().ok()).ok_or_else(
        || {
            ::serde::de::Error::custom("")
        },
    )?;
    let long = split.next().and_then(|long| long.parse().ok()).ok_or_else(
        || {
            ::serde::de::Error::custom("")
        },
    )?;
    Ok(Coordinate { lat, long })
}
