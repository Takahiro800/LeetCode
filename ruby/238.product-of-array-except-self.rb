# @param {Integer[]} nums
# @return {Integer[]}
def product_except_self(nums)
  prefix = []
  total_product = 1

  nums.each do |num|
    prefix << total_product
    total_product *= num
  end

  reverse_nums = nums.reverse
  total_product = 1
  reverse_suffix = []

  reverse_nums.each do |num|
    reverse_suffix << total_product
    total_product *= num
  end

  suffix = reverse_suffix.reverse

  nums.map.each_with_index { |_, i| prefix[i] * suffix[i] }
end
