$LOAD_PATH.unshift(File.expand_path('../vendor/bundle', __FILE__))
require 'bundler/setup'

require 'alphred'

require_relative 'forecaster'
require_relative 'location'
require_relative 'spark'

ICONS = {
  'clear-day'           => 'Sun',
  'clear-night'         => 'Moon',
  'rain'                => 'Cloud-Rain',
  'snow'                => 'Cloud-Snow',
  'sleet'               => 'Cloud-Snow-Alt',
  'wind'                => 'Wind',
  'fog'                 => 'Cloud-Fog',
  'cloudy'              => 'Cloud',
  'partly-cloudy-day'   => 'Cloud-Sun',
  'partly-cloudy-night' => 'Cloud-Moon',
}

Precipitation = Struct.new(:intensity, :probability) do
  def self.from_forecast(forecast)
    self.new(*forecast.values_at('precipIntensity', 'precipProbability'))
  end

  def human_intensity
    case intensity
    when 0...0.002
      'no'
    when 0.002...0.017
      'very light'
    when 0.017...0.1
      'light'
    when 0.1...0.4
      'moderate'
    else
      'heavy'
    end
  end

  def to_s
    "#{(probability*100).to_i}% chance of #{human_intensity} rain."
  end
end

query = ARGV.shift || ''
location = if query.empty?
             if ENV['DEFAULT_LAT_LONG'].to_s.empty?
               Location.from_ip
             else
               lat, long = ENV['DEFAULT_LAT_LONG'].to_s.split(?,).map(&:to_f)
               Location.new(ENV['DEFAULT_LOCATION'].to_s, lat, long)
             end
           else
             Location.new(query)
           end
forecast = Forecaster.forecast(location)

items = Alphred::Items.new

items << Alphred::Item.new(
  arg: "#{location.lat.round(4)},#{location.long.round(4)}",
  valid: true,
  title: location.name,
  icon: 'icons/forecast.png',
)

currently = forecast['currently']
precip = Precipitation.from_forecast(currently)
subtitle = [ "#{currently['temperature'].round}°" ]
subtitle << "Feels like #{currently['apparentTemperature'].round}°"
subtitle << precip.to_s if precip.probability > 0
items << Alphred::Item.new(
  title: currently['summary'],
  subtitle: subtitle.join(' · '),
  icon: "icons/#{ICONS[currently['icon']]}.png",
)

minutely = forecast['minutely']
if minutely
  intensity = minutely['data'].map {|m| m['precipIntensity'] }
  intensity = intensity.select.with_index {|_,i| i % 4 == 0 }
  min, max = intensity.minmax
  intensity = intensity.map {|i| 1000 * i }

  subtitle = ["#{min.round(3)}\" #{Spark.new(intensity)} #{max.round(3)}\""]

  probability = minutely['data'].map {|m| (100 * m['precipProbability']).round }
  probability = probability.select.with_index {|_,i| i % 5 == 0 }
  min, max = probability.minmax

  subtitle << "#{min}% #{Spark.new(probability, max: 100)} #{max}%"

  items << Alphred::Item.new(
    title: minutely['summary'],
    subtitle: subtitle.join(' · '),
    icon: "icons/#{ICONS[minutely['icon']]}.png",
  )
end

data = forecast['daily']['data'][0]
subtitle = [ "Low: #{data['apparentTemperatureMin'].round}°",
             "High: #{data['apparentTemperatureMax'].round}°" ]
precip = Precipitation.from_forecast(data)
subtitle << precip.to_s if precip.probability > 0

items << Alphred::Item.new(
  title: data['summary'],
  subtitle: subtitle.join(' · '),
  icon: "icons/#{ICONS[data['icon']]}.png",
)

forecast['daily']['data'][1..5].each do |data|
  wday = Time.at(data['time']).strftime('%A')
  precip = Precipitation.from_forecast(data)

  subtitle = [ "Low: #{data['apparentTemperatureMin'].round}°",
               "High: #{data['apparentTemperatureMax'].round}°" ]
  subtitle << precip.to_s if precip.probability > 0

  items << Alphred::Item.new(
    title: "#{wday} - #{data['summary']}",
    subtitle: subtitle.join(' · '),
    icon: "icons/#{ICONS[data['icon']]}.png",
  )
end

puts items.to_json
