# @param {Integer[]} nums
# @return {Integer[][]}
def three_sum(nums)
  sorted_nums = nums.sort

  ans = []
  sorted_nums.each_with_index.each do |num, index|
    return ans if num > 0
    next if index > 0 && num == sorted_nums[index - 1]

    left = index + 1
    right = sorted_nums.size - 1

    while left < right
      if (num + sorted_nums[left] + sorted_nums[right]).zero?
        ans << [num, sorted_nums[left], sorted_nums[right]]

        while sorted_nums[left] == sorted_nums[left + 1]
          left += 1
        end
        left += 1

        next
      elsif (num + sorted_nums[left] + sorted_nums[right]).positive?
        right -= 1
      else
        left += 1
      end
    end
  end

  ans
end
