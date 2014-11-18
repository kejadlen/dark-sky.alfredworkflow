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
