# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def search(nums, target)
  left = 0
  right = nums.size - 1

  while left <= right
    return left if nums[left] == target
    return right if nums[right] == target

    mid = ((left + right) / 2).floor
    return mid if nums[mid] == target

    if target < nums[mid]
      right = mid - 1
    else
      left = mid + 1
    end
  end

  -1
end

# pub fn search(nums: Vec<i32>, target: i32) -> i32 {
#   let mut left = 0;
#   let mut right = nums.len() - 1;

#   while left <= right {
#     if nums[left] == target {
#       return left as i32;
#     }
#     if nums[right] == target {
#       return right as i32;
#     }

#     let mid = (left + right) / 2;
#     if nums[mid] == target {
#       return mid as i32;
#     }

#     if target < nums[mid] {
#       right = mid - 1;
#     } else {
#       left = mid + 1;
#     }
#   }

#   -1
# }
