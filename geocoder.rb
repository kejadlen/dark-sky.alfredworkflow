require 'json'
require 'open-uri'
require 'uri'

require_relative 'config'

Geocoder = Struct.new(:api_key) do
  def self.geocode(location)
    geocoder.geocode(location)
  end

  def self.geocoder
    return @geocoder if defined?(@geocoder)

    @geocoder = self.new(Forecast::Config['GOOGLE_API_KEY'])
  end

  def geocode(location)
    url = 'https://maps.googleapis.com/maps/api/geocode/json'
    query = URI.encode_www_form(address: location, api_key: api_key)
    response = JSON.load(open("#{url}?#{query}"))
    result = response['results'][0]

    name = result['formatted_address']
    location = result['geometry']['location']
    lat, long = location.values_at('lat', 'lng')

    [name, lat, long]
  end
end
