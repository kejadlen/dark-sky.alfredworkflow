require 'delegate'
require 'erb'
require 'yaml'

class Items < DelegateClass(Array)
  attr_reader :items

  def initialize
    @items = []
    super(@items)
  end

  def to_s
    ERB.new(<<-XML).result(binding)
<?xml version="1.0"?>
<items>
  <%= items.map {|item| item.to_s.split("\n").map {|line| '  ' << line }}.join("\n").strip %>
</items>
    XML
  end
end

class Item
  attr_reader *%i[ uid arg valid
                   title subtitle icon ]
  def initialize(**kwargs)
    @uid = kwargs.fetch(:uid).to_s.encode(xml: :attr)
    @arg = kwargs[:arg].to_s.encode(xml: :attr)
    @valid = kwargs.fetch(:valid, false) ? 'yes' : 'no'
    @title = kwargs.fetch(:title).encode(xml: :text)
    @subtitle = kwargs[:subtitle] && kwargs[:subtitle].encode(xml: :text)
    @icon = kwargs[:icon] && kwargs[:icon].encode(xml: :text)
  end

  def to_s
    ERB.new(<<-XML, nil, '%>').result(binding)
<item arg=<%= arg %> uid=<%= uid %> valid="<%= valid %>">
  <title><%= title %></title>
% if subtitle
  <subtitle><%= subtitle %></subtitle>
% end
% if icon
  <icon><%= icon %></icon>
% end
</item>
    XML
  end
end

module Alfred
  class Config
    def self.[](key)
      config[key]
    end

    def self.[]=(key, value)
      config[key] = value
    end

    def self.config
      return @config if defined?(@config)

      bundle_id = `/usr/libexec/PlistBuddy info.plist -c 'print :bundleid'`.strip
      @config = self.new(bundle_id)
    end

    WORKFLOW_DATA = '~/Library/Application Support/Alfred 2/Workflow Data/'

    attr_reader :path
    attr_accessor :config

    def initialize(bundle_id)
      dir = File.expand_path(File.join(WORKFLOW_DATA, bundle_id))
      Dir.mkdir(dir) unless Dir.exist?(dir)

      @path = File.join(dir, 'config.yml')
      @config = File.exist?(@path) ? YAML.load_file(@path) : {}
    end

    def [](key)
      config.fetch(key) { '' }
    end

    def []=(key, value)
      config[key] = value
      File.write(path, YAML.dump(config))
    end
  end
end
