# Dark Sky Workflow for Alfred

![screenshot][screenshot]

[screenshot]: http://i.imgur.com/lbA9fPW.png

# Requirements

- [Alfred](http://www.alfredapp.com/)
- [Alfred Powerpack](http://www.alfredapp.com/powerpack/)

# Installation

Download and install the [workflow][download].

[download]: https://github.com/kejadlen/dark-sky.alfredworkflow/releases/download/v3.0.2/dark-sky.alfredworkflow

These environment variables can be [configured in Alfred][env-vars]:

- `DARK_SKY_API_KEY`: Get an API key [here][dark-sky-api-key].
- `GOOGLE_API_KEY`: Get an API key [here][google-api-key]. (Used for geocoding
  queries. *This can be omitted if you only want the forecast for the current
  location*.)
- `FORECAST_UNITS`: Defaults to `auto`, which sets the units based on the
  location. Use `si` for Celsius and `us` for Fahrenheit.
- `DEFAULT_LAT_LONG`: Set this to override IP geolocation. Ex: `47.7396,-122.3426` for Seattle.
- `DEFAULT_LOCATION`: Used for displaying the location name when using `DEFAULT_LAT_LONG`.
- `LIGHT_ICONS`: `true` gives white icons, `false` gives black icons.

[env-vars]: https://www.alfredapp.com/help/workflows/advanced/variables/
[dark-sky-api-key]: https://darksky.net/dev/register
[google-api-key]: https://developers.google.com/maps/documentation/geocoding/#api_key

# Attributions

- [Climacons](http://adamwhitcroft.com/climacons/)
- [Dark Sky API](https://darksky.net/dev/docs)
- [Google Geocoding API](https://developers.google.com/maps/documentation/geocoding/)
- [ipinfo.io](http://ipinfo.io/)
