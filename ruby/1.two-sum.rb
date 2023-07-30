# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  len = nums.length

  (0...len).each do |i|
    (i + 1...len).each do |j|
      return [i, j] if nums[i] + nums[j] == target
    end
  end
end
