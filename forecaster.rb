require 'json'
require 'open-uri'

Forecaster = Struct.new(:api_key) do
  def self.forecast(location)
    forecaster.forecast(location)
  end

  def self.forecaster
    return @forecaster if defined?(@forecaster)

    @forecaster = self.new(ENV['DARK_SKY_API_KEY'].to_s)
  end

  def forecast(location)
    lat, long = location.lat, location.long
    units = ENV['FORECAST_UNITS'].to_s
    url = "https://api.darksky.net/forecast/#{api_key}/#{lat},#{long}?units=#{units}"
    response = JSON.load(open(url))
  end
end

