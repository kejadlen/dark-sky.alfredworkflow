## 0.0.5 - 2014.11.22
### Changed
- Use `forecast-config` for managing API keys.
- Fixed bug when precipitation intensity/probability was all 0's.

## 0.0.4 - 2014.11.21
### Added
- Sparklines for precipitation intensity and probability for the next hour
  (where applicable) and day.

### Changed
- Bugfix for when `DEFAULT_LAT_LONG` is set and `DEFAULT_LOCATION` is not.

## 0.0.3 - 2014.11.19
### Added
- Forecast now uses units appropriate to the location.

### Changed
- Fix `DEFAULT_LAT_LONG`.

## 0.0.2 - 2014.11.19
### Changed
- Remove minutely result for non-US locations since Forecast doesn't have this
  data.

## 0.0.1 - 2014.11.18
### Added
- Initial release
