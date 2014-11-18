class Spark
  TICKS = %w[▁ ▂ ▃ ▄ ▅ ▆ ▇ █]

  attr_reader :data

  def initialize(*data)
    @data = data
  end

  def to_s
    min = data.min
    max = data.max
    range = (max - min).to_f
    graph = data.map {|i| TICKS[(TICKS.size - 1) * (i - min) / range] }.join
    "#{min} #{graph} #{max}"
  end
end
