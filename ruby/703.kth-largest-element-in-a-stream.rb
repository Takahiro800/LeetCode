# frozen_string_literal: true

class KthLargest
  def initialize(max_size, nums)
    @max_size = max_size
    @nums = nums.sort.reverse[0..k]
  end

  def add(val)
    if (i = @nums.bsearch_index { |num| num < val })
      @nums.insert(i, val)
    else
      @nums.push(val)
    end

    @nums.pop if @nums.length > @max_size
    @nums.last
  end
end
