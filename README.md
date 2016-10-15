# Dark Sky Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/lbA9fPW.png

# Requirements

- [Alfred](http://www.alfredapp.com/)
- [Alfred Powerpack](http://www.alfredapp.com/powerpack/)
- OS X Mavericks

# Installation

Download and install the [workflow][download].

[download]: https://github.com/kejadlen/dark-sky.alfredworkflow/releases/download/v2.0.0/DarkSky.alfredworkflow

These environment variables can be [configured in Alfred][env-vars]:

- `DARK_SKY_API_KEY`: Get an API key [here][dark-sky-api-key].
- `GOOGLE_API_KEY`: Get an API key [here][google-api-key]. (Used for geocoding
  queries. *This can be omitted if you only want the forecast for the current
  location*.)
- `FORECAST_UNITS`: Defaults to `auto`, which sets the units based on the
  location. Use `si` for Celsius and `us` for Fahrenheit.
- `DEFAULT_LAT_LONG`: Set this to override IP geolocation. Ex: `47.7396,-122.3426` for Seattle.
- `DEFAULT_LOCATION`: Used for displaying the location name when using `DEFAULT_LAT_LONG`.

[env-vars]: https://www.alfredapp.com/help/workflows/advanced/variables/
[dark-sky-api-key]: https://darksky.net/dev/register
[google-api-key]: https://developers.google.com/maps/documentation/geocoding/#api_key

# TODO

- Handle errors gracefully
- Caching? (Probably unnecessary...)
- Use `Accept-Encoding: gzip` for Dark Sky API calls

# Attributions

- [Climacons](http://adamwhitcroft.com/climacons/)
- [Dark Sky API](https://darksky.net/dev/docs)
- [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding/)
- [ipinfo.io](http://ipinfo.io/)
