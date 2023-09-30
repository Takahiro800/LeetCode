# @param {Integer[]} numbers
# @param {Integer} target
# @return {Integer[]}
def two_sum(numbers, target)
  numbers.each_with_index.each_with_object({}) do |(num, index), hash|
    if hash[target - num]
      return [hash[target - num], index + 1]
    end

    hash[num] = index + 1
  end
end
