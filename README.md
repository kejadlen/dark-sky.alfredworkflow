# Forecast Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/enspDWu.png

# Requirements

- [Alfred](http://www.alfredapp.com/)
- [Alfred Powerpack](http://www.alfredapp.com/powerpack/)
- OS X Mavericks

# Installation

Download and install the [workflow][download]. Edit the script filter to add your location and API keys:

[download]: https://github.com/kejadlen/forecast.alfredworkflow/releases/download/0.0.1/Forecast.alfredworkflow

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

- Sparklines for precipitation?
- Make installation easier
- Handle errors gracefully
- Caching? (Probably unnecessary...)

# Attributions

- Icons from [Climacons](http://adamwhitcroft.com/climacons/)
