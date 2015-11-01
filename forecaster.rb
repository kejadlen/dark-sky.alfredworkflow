require 'json'
require 'open-uri'

require_relative 'config'

Forecaster = Struct.new(:api_key) do
  def self.forecast(location)
    forecaster.forecast(location)
  end

  def self.forecaster
    return @forecaster if defined?(@forecaster)

    @forecaster = self.new(Forecast::Config['FORECAST_API_KEY'])
  end

  def forecast(location)
    lat, long = location.lat, location.long
    units = Forecast::Config['FORECAST_UNITS']
    url = "https://api.forecast.io/forecast/#{api_key}/#{lat},#{long}?units=#{units}"
    response = JSON.load(open(url))
  end
end

