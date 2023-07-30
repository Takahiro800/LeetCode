# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  nums.each_with_index.each_with_object({}) do |(n, i), hash|
    return [hash[target - n], i] if hash[target - n]

    hash[n] = i
  end
end
