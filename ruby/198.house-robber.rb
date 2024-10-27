# @param {Integer[]} nums
# @return {Integer}
def rob(nums)
  prev_a = 0
  prev_b = 0
  nums.each do |num|
    a = prev_b + num
    b = [prev_a, prev_b].max

    prev_a = a
    prev_b = b
  end

  [prev_a, prev_b].max
end
