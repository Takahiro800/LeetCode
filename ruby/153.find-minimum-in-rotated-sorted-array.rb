# @param {Integer[]} nums
# @return {Integer}
def find_min(nums)
  left = 0
  right = nums.length - 1

  return nums[left] if nums[left] < nums[right]

  while left + 1 < right
    mid = (left + right) / 2

    if nums[mid] > nums[left]
      left = mid
    else
      right = mid
    end
  end

  [nums[left], nums[right]].min
end
