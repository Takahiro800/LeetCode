class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {Integer[]} nums
# @return {TreeNode}
def sorted_array_to_bst(nums)
  return if nums.empty?

  mid = nums.length / 2
  node = TreeNode.new(nums[mid])
  node.left = sorted_array_to_bst(nums[...mid])
  node.right = sorted_array_to_bst(nums[mid + 1..])

  node
end
