require_relative 'geocoder'

class Location
  attr_accessor :name, :lat, :long, :geocoder

  def initialize(name, lat=nil, long=nil, geocoder=Geocoder)
    @name, @lat, @long, @geocoder = name, lat, long, geocoder

    geocode! unless lat && long
  end

  def geocode!
    self.name, self.lat, self.long = geocoder.geocode(name)
  end
end

