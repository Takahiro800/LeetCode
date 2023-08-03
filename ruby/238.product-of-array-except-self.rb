# @param {Integer[]} nums
# @return {Integer[]}
def product_except_self(nums)
  prefix = prefix_array(nums)
  suffix = prefix_array(nums.reverse).reverse

  nums.map.each_with_index { |_, i| prefix[i] * suffix[i] }
end

def prefix_array(nums)
  total_product = 1

  nums.each_with_object([]) do |n, arr|
    arr << total_product
    total_product *= n
  end
end
