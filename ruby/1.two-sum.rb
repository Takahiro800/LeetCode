# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  hash = {}

  nums.each_with_index do |num, i|
    return [i, hash[target - num]] if hash[target - num]

    hash[num] = i
  end
end
