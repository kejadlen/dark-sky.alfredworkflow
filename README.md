# Forecast Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/enspDWu.png

# Requirements

- [Alfred](http://www.alfredapp.com/)
- [Alfred Powerpack](http://www.alfredapp.com/powerpack/)
- OS X Mavericks

# Installation

Install the workflow. Edit the script filter to add your location and API keys:

- FORECAST_API_KEY: Get an API key [here][forecast-api-key].
- GOOGLE_API_KEY: Get an API key [here][google-api-key]. (Only required if
  getting the forecast for an arbitrary location.)
- DEFAULT_LOCATION
- DEFAULT_LAT_LONG: Required if GOOGLE_API_KEY is not used. Format: `lat,long`.

[forecast-api-key]: https://developer.forecast.io/register
[google-api-key]: https://developers.google.com/maps/documentation/geocoding/#api_key

# TODO

- Sparklines for precipitation?
- Make installation easier
- Handle errors gracefully
- Caching? (Probably unnecessary...)

# Attributions

- Icons from [Climacons](http://adamwhitcroft.com/climacons/)
