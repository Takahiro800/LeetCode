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
  min_depth_recursion(root, 0)
end

def min_depth_recursion(node, current_depth)
  return current_depth if node.nil?

  left_depth = min_depth_recursion(node.left, current_depth + 1)
  right_depth = min_depth_recursion(node.right, current_depth + 1)

  if !node.left.nil? && !node.right.nil?
    [left_depth, right_depth].min
  else
    [left_depth, right_depth].max
  end
end
