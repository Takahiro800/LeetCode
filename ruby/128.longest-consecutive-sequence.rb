# @param {Integer[]} nums
# @return {Integer}
def longest_consecutive(nums)
  set = Set.new(nums)

  set.reduce(0) do |longest, num|
    unless set.include?(num - 1)
      length = 0

      while set.include?(num + length)
        length += 1
      end
      next (longest > length ? longest : length)
    end

    longest
  end
end
