# @param {Integer[]} nums
# @return {Boolean}
def contains_duplicate(nums)
  hashset = {}

  nums.each do |num|
    return true if hashset.key?(num)

    hashset[num] = true
  end

  false
end
