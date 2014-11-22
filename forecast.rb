require 'delegate'
require 'erb'
require 'yaml'

require_relative 'alfred'
require_relative 'forecaster'
require_relative 'location'
require_relative 'spark'

ICONS = {
  'clear-day' => 'Sun',
  'clear-night' => 'Moon',
  'rain' => 'Cloud-Rain',
  'snow' => 'Cloud-Snow',
  'sleet' => 'Cloud-Snow-Alt',
  'wind' => 'Wind',
  'fog' => 'Cloud-Fog',
  'cloudy' => 'Cloud',
  'partly-cloudy-day' => 'Cloud-Sun',
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
             lat, long = Alfred::Config['DEFAULT_LAT_LONG'].split(?,).map(&:to_f)
             Location.new(Alfred::Config['DEFAULT_LOCATION'], lat, long)
           else
             Location.new(query)
           end
forecast = Forecaster.forecast(location)

items = Items.new

items << Item.new(
  uid: :location,
  arg: "#{location.lat.round(4)},#{location.long.round(4)}",
  valid: true,
  title: location.name,
  icon: 'icons/forecast.ico',
)

currently = forecast['currently']
precip = Precipitation.from_forecast(currently)
subtitle = [ "#{currently['temperature'].round}°" ]
subtitle << "Feels like #{currently['apparentTemperature'].round}°"
subtitle << precip.to_s if precip.probability > 0
items << Item.new(
  uid: :currently,
  title: currently['summary'],
  subtitle: subtitle.join(' · '),
  icon: "icons/#{ICONS[currently['icon']]}.png",
)

minutely = forecast['minutely']
if minutely
  intensity = minutely['data'].map {|m| 1000 * m['precipIntensity'] }
  intensity = intensity.select.with_index {|_,i| i % 5 == 0 }
  min, max = intensity.minmax

  subtitle = ["#{min.round}\" #{Spark.new(intensity)} #{max.round}\""]

  probability = minutely['data'].map {|m| (100 * m['precipProbability']).round }
  probability = probability.select.with_index {|_,i| i % 5 == 0 }
  min, max = probability.minmax

  subtitle << "#{min}% #{Spark.new(probability, max: 100)} #{max}%"

  items << Item.new(
    uid: :minutely,
    title: minutely['summary'],
    subtitle: subtitle.join(' · '),
    icon: "icons/#{ICONS[minutely['icon']]}.png",
  )
end

hourly = forecast['hourly']

intensity = hourly['data'].map {|m| 1000 * m['precipIntensity'] }
intensity = intensity.select.with_index {|_,i| i % 4 == 0 }
min, max = intensity.minmax

subtitle = ["#{min.round}\" #{Spark.new(intensity)} #{max.round}\""]

probability = hourly['data'].map {|m| (100 * m['precipProbability']).round }
probability = probability.select.with_index {|_,i| i % 4 == 0 }
min, max = probability.minmax

subtitle << "#{min}% #{Spark.new(probability, max: 100)} #{max}%"

items << Item.new(
  uid: :hourly,
  title: hourly['summary'],
  subtitle: subtitle.join(' · '),
  icon: "icons/#{ICONS[hourly['icon']]}.png",
)

forecast['daily']['data'][1..6].each do |data|
  wday = Time.at(data['time']).strftime('%A')
  precip = Precipitation.from_forecast(data)

  subtitle = [ "Low: #{data['apparentTemperatureMin'].round}°",
               "High: #{data['apparentTemperatureMax'].round}°" ]
  subtitle << precip.to_s if precip.probability > 0

  items << Item.new(
    uid: wday,
    title: "#{wday} - #{data['summary']}",
    subtitle: subtitle.join(' · '),
    icon: "icons/#{ICONS[data['icon']]}.png",
  )
end

puts items.to_s
