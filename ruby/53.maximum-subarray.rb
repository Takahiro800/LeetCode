# @param {Integer[]} nums
# @return {Integer}
def max_sub_array(nums)
  current_sum = ans = nums[0]
  nums[1..].each do |num|
    current_sum = [current_sum + num, num].max
    ans = [ans, current_sum].max
  end

  ans
end
