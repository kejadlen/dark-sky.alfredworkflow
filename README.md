# Forecast Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/mxGnovo.png

# Requirements

- [Alfred](http://www.alfredapp.com/)
- [Alfred Powerpack](http://www.alfredapp.com/powerpack/)
- OS X Mavericks

# Installation

Download and install the [workflow][download].

[download]: https://github.com/kejadlen/forecast.alfredworkflow/releases/download/0.0.5/Forecast.alfredworkflow

Run `forecast-config VALUE` to set API keys and the default location:

- `FORECAST_API_KEY`: Get an API key [here][forecast-api-key].
- `GOOGLE_API_KEY`: Get an API key [here][google-api-key]. (Used for geocoding.
  If you never need to search for a location, this can be omitted by using
  `DEFAULT_LAT_LONG`.)
- `DEFAULT_LOCATION`: Ex. "Seattle, WA".
- `DEFAULT_LAT_LONG`: Only required if `GOOGLE_API_KEY` is unavailable, since
  `DEFAULT_LOCATION` can't be geocoded. Format: `lat,long`.

[forecast-api-key]: https://developer.forecast.io/register
[google-api-key]: https://developers.google.com/maps/documentation/geocoding/#api_key

# TODO

- Handle errors gracefully
- Caching? (Probably unnecessary...)
- Use `Accept-Encoding: gzip` for Forecast calls

# Attributions

- [Climacons](http://adamwhitcroft.com/climacons/)
- [Forecast API](https://developer.forecast.io/docs/v2)
- [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding/)
