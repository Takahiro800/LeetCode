# @param {Integer[]} nums
# @return {Integer}
def rob(nums)
  return nums[0] if nums.size == 1

  rob_first = rob_linear(nums[...nums.size - 1])
  not_rob_first = rob_linear(nums[1..])

  [rob_first, not_rob_first].max
end

def rob_linear(nums)
  a, b = nums.reduce([0, 0]) do |(prev_a, prev_b), num|
    [prev_b + num, [prev_a, prev_b].max]
  end

  [a, b].max
end
