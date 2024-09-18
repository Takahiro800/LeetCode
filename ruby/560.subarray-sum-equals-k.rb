# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def subarray_sum(nums, k)
  sum = 0
  sum_count = Hash.new(0)
  sum_count[0] = 1

  ans = 0

  nums.each do |num|
    sum += num
    target = sum - k
    ans += sum_count[target] if sum_count.key?(target)

    sum_count[sum] += 1
  end

  ans
end
