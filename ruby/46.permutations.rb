# frozen_string_literal: true

# @param {Integer[]} nums
# @return {Integer[][]}
def permute(nums)
  used = Array.new(nums.size, false)
  ans = []
  back_track([], used, nums, ans)
  ans
end

def back_track(path, used, nums, ans)
  if path.size == nums.size
    ans << path.dup
    return ans
  end

  nums.each_with_index do |num, index|
    next if used[index]

    used[index] = true
    path << num
    back_track(path, used, nums, ans)

    used[index] = false
    path.pop
  end
end
