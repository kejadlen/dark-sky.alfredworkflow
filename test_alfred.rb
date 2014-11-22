require 'minitest/autorun'

require_relative 'alfred'

class TestConfig < Minitest::Test
  def setup
    @config = Alfred::Config.new('com.kejadlen.test')
  end

  def teardown
    File.delete(@config.path) if File.exist?(@config.path)
  end

  def test_config
    assert_nil @config[:foo]

    @config[:foo] = 123
    assert_equal 123, @config[:foo]
    assert File.exist?(@config.path)
    assert_equal '{:foo=>123}', File.read(@config.path)
  end
end
