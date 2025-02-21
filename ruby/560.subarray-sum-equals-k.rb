# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def subarray_sum(nums, k)
  sum_count = Hash.new(0)
  sum_count[0] = 1

  sum = 0
  nums.reduce(0) do |ans, num|
    sum += num
    ans += sum_count[sum - k]
    sum_count[sum] += 1

    ans
  end
end
