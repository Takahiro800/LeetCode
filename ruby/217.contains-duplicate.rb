# @param {Integer[]} nums
# @return {Boolean}

require "set"

def contains_duplicate(nums)
  nums.each_with_object(Set.new) do |num, set|
    return true unless set.add?(num)
  end and false
end

puts contains_duplicate([])
