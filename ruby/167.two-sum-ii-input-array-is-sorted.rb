# @param {Integer[]} numbers
# @param {Integer} target
# @return {Integer[]}
def two_sum(numbers, target)
  left = 0
  right = numbers.length - 1

  while left < right
    return [left + 1, right + 1] if numbers[left] + numbers[right] == target

    if numbers[left] + numbers[right] > target
      right -= 1
    else
      left += 1
    end
  end
end
