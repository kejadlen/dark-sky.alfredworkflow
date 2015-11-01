$LOAD_PATH.unshift(File.expand_path("../vendor/bundle", __FILE__))
require "bundler/setup"

require "alphred"

module Forecast
  Config = Alphred::Config.load(
    FORECAST_API_KEY: nil,
    GOOGLE_API_KEY: nil,
    FORECAST_UNITS: nil,
    DEFAULT_LOCATION: nil,
    DEFAULT_LAT_LONG: nil,
  )
end
