# Dark Sky Workflow for Alfred

With [Dark Sky shutting down their API][dark-sky-incredible-journey], this has
been replaced by a [Pirate Weather workflow][pirate-weather-workflow].

[dark-sky-incredible-journey]: https://blog.darksky.net/
[pirate-weather-workflow]: https://github.com/kejadlen/pirate-weather.alfredworkflow/

## Migration

[Obtain a Pirate Weather API key][pirate-weather-getting-started] and set
`PIRATE_WEATHER_API_KEY` in the workflow environment variables.

[pirate-weather-getting-started]: https://pirateweather.net/getting-started

## Notes

You can still use Dark Sky as a data source until the API is actually turned
off by setting the `PIRATE_WEATHER_ENDPOINT` environment variable to
`api.darksky.net`.
