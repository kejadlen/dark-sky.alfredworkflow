use serde::Deserialize;

#[derive(Debug)]
pub struct Coordinate {
    pub lat: f64,
    pub long: f64,
}

impl<'de> Deserialize<'de> for Coordinate {
    fn deserialize<D>(deserializer: D) -> Result<Coordinate, D::Error>
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
}

#[test]
fn test_deserializing_coordinate() {
    let c: Result<Coordinate, _> = ::serde_json::from_str("\"\"");
    assert!(c.is_err());

    let c: Result<Coordinate, _> = ::serde_json::from_str("\"123\"");
    assert!(c.is_err());

    let c: Result<Coordinate, _> = ::serde_json::from_str("\"123,\"");
    assert!(c.is_err());

    let c: Coordinate = ::serde_json::from_str("\"123,-123\"").unwrap();
    assert_eq!(c.lat, 123.0);
    assert_eq!(c.long, -123.0);
}
