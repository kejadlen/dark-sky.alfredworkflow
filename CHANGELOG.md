## Unreleased
### Added
- `darksky` keyword trigger

### Changed
- Switched to [Dark Sky from Forecast.io][forecast-to-dark-sky]
  - Renamed workflow to `dark-sky.alfredworkflow`
  - Use `DARK_SKY_API_KEY` instead of `FORECAST_API_KEY`

[forecast-to-dark-sky]: http://blog.darksky.net/introducing-darksky-net/

## [1.0.8] - 2016-08-04
### Added
- Selecting a daily item now goes to the day on Forecast.io.

## [1.0.7] - 2016-07-28
### Fixed
- Re-release to fix vendoring.

## [1.0.6] - 2016-07-27
### Fixed
- Off by one error on which days to display.
- Re-enable going to the Forecast.io page on all items.

## [1.0.5] - 2016-07-20
### Changed
- Update to Alfred 3: use built-in configuration for API keys.
- Modified the data display for the current day.

### Removed
- Don't allow sorting of the items.

## [1.0.4] - 2016-04-17
### Fixed
- Update dependencies.

## [1.0.3] - 2016-04-05
### Added
- Delay fetching results until after the last character is typed.

### Fixed
- Actually handle when there isn't a config file.

## [1.0.2] - 2015-11-01
### Changed
- Update Alphred to handle a crash when there isn't a config file.

## [1.0.0] - 2015-11-01
### Changed
- Use [Alphred](https://github.com/kejadlen/alphred).

## [0.0.8] - 2015-05-31
### Added
- Re-add support for `DEFAULT_LAT_LONG` and `DEFAULT_LOCATION`.

## [0.0.7] - 2015-01-05
### Added
- The current location is retrieved via IP geolocation rather than set in the
  workflow configuration.
- Fixed opening the forecast in the browser.

### Removed
- Support for `DEFAULT_LAT_LONG` and `DEFAULT_LOCATION` has been deprecated in favor
  of getting the current location from the IP.

## [0.0.6] - 2014-12-13
### Added
- Add option to force Celsius/Fahrenheit using `FORECAST_UNITS`.

## [0.0.5] - 2014-11-22
### Changed
- Use `forecast-config` for managing API keys.
- Fixed bug when precipitation intensity/probability was all 0's.

## [0.0.4] - 2014-11-21
### Added
- Sparklines for precipitation intensity and probability for the next hour
  (where applicable) and day.

### Changed
- Bugfix for when `DEFAULT_LAT_LONG` is set and `DEFAULT_LOCATION` is not.

## [0.0.3] - 2014-11-19
### Added
- Forecast now uses units appropriate to the location.

### Changed
- Fix `DEFAULT_LAT_LONG`.

## [0.0.2] - 2014-11-19
### Changed
- Remove minutely result for non-US locations since Forecast doesn't have this
  data.

## [0.0.1] - 2014-11-18
### Added
- Initial release
