require 'json'
require 'open-uri'

require_relative 'geocoder'

class Location
  def self.from_ip(ip=nil)
    url = ['http://ipinfo.io', ip, 'json'].compact.join(?/)
    response = JSON.load(open(url))

    lat, long = response['loc'].split(?,).map(&:to_f)
    name = "#{response['city']}, #{response['region']}"
    self.new(name, lat, long)
  end

  attr_accessor :name, :lat, :long, :geocoder

  def initialize(name, lat=nil, long=nil, geocoder=Geocoder)
    @name, @lat, @long, @geocoder = name, lat, long, geocoder

    geocode! unless lat && long
  end

  def geocode!
    self.name, self.lat, self.long = geocoder.geocode(name)
  end
end

