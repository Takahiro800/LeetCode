# frozen_string_literal: true

# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def search_insert(nums, target)
  return 0 if target <= nums[0]

  left = 0
  right = nums.size

  while right - left > 1
    mid = left + (right - left) / 2

    if target == nums[mid]
      return mid
    elsif target < nums[mid]
      right = mid
    else
      left = mid
    end
  end

  nums[left] == target ? left : right
end
