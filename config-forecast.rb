require_relative 'alfred'

OPTIONS = %w[ FORECAST_API_KEY
              GOOGLE_API_KEY
              DEFAULT_LOCATION
              DEFAULT_LAT_LONG ]

input = ARGV.shift || ''

items = Items.new
OPTIONS.each do |option|
  title = if input.empty?
            "Unset #{option}"
          else
            "Set #{option} to #{input}"
          end
  items << Item.new(
    uid: option,
    arg: "Alfred::Config['#{option}'] = '#{input}'",
    valid: true,
    title: title,
    subtitle: Alfred::Config[option],
  )
end

puts items.to_s
