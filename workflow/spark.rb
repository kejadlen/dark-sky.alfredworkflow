class Spark
  # TICKS = %w[▁ ▂ ▃ ▄ ▅ ▆ ▇ █] # Alfred doesn't render the last bar correctly
                                # for some reason...
  TICKS = %w[▁ ▂ ▃ ▄ ▅ ▆ ▇]

  attr_reader :data, :min, :max

  def initialize(data, **kwargs)
    @data = data.map(&:round)
    @min = kwargs.fetch(:min) { 0 }
    @max = [(kwargs.fetch(:max) { data.max }).to_f, 1.0].max
  end

  def to_s
    data.map {|i| TICKS[(TICKS.size - 1) * (i - min) / max] }.join
  end
end
