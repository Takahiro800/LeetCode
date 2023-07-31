# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer[]}
def top_k_frequent(nums, k)
  hash = nums.tally
  counter_array = Array.new(hash.values.max + 1) { [] }

  hash.each_pair { |key, value| counter_array[value].append(key) }
  counter_array.flatten.last(k)
end
