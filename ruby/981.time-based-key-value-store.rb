class TimeMap
  def initialize
    @store = {}
  end

  def set(key, value, timestamp)
    @store[key] = [] unless @store.keys.include?(key)

    @store[key] << { value: value, timestamp: timestamp }
  end

  def get(key, timestamp)
    return '' unless @store.key?(key)

    set = @store[key]
    return '' if timestamp < set[0][:timestamp]
    return set.last[:value] if timestamp >= set.last[:timestamp]

    left = 0
    right = set.length

    while left < right
      mid = (left + right) / 2
      return set[mid][:value] if set[mid][:timestamp] == timestamp

      if timestamp < set[mid][:timestamp]
        right = mid
      else
        left = mid + 1
      end
    end

    set[left - 1][:value]
  end
end
