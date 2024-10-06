class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {TreeNode} root
# @return {Integer}
def min_depth(root)
  return 0 if root.nil?

  left_depth = min_depth(root.left)
  right_depth = min_depth(root.right)

  if left_depth.positive? && right_depth.positive?
    [left_depth, right_depth].min + 1
  else
    left_depth + right_depth + 1
  end
end
