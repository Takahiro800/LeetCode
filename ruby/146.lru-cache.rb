class LRUCache
  attr_accessor :hash
  attr_reader :capacity

  def initialize(capacity)
    @capacity = capacity
    @hash = Hash.new
  end

  def get(key)
    if hash[key]
      value = hash.delete(key)
      return hash[key] = value
    end

    -1
  end

  def put(key, value)
    if hash[key]
      hash.delete(key)
    end

    if hash.size >= capacity
      hash.shift
    end

    hash[key] = value
  end
end
