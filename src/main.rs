#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod errors;

use errors::*;

quick_main!(run);

fn run() -> Result<()> {
    let ip_info: IPInfo = reqwest::get("https://ipinfo.io/json")?.json().map_err(
        Error::from,
    )?;
    println!("{:?}", ip_info);
    Ok(())
}

#[derive(Debug, Deserialize)]
struct IPInfo {
    loc: String,
    city: String,
    region: String,
}
