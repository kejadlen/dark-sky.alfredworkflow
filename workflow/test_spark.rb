require 'minitest/autorun'

require_relative 'spark'

class TestSpark < Minitest::Test
  def test_div_by_zero
    spark = Spark.new([0, 0, 0])
    assert_equal '▁▁▁', spark.to_s
  end
end
