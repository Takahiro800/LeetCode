# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer[]}
def two_sum(nums, target)
  nums.each_with_index.each_with_object({}) do |(num, i), stack|
    return [i, stack[target - num]] if stack[target - num]

    stack[num] = i
  end
end
