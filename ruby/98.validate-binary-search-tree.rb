class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {TreeNode} root
# @return {Boolean}
def is_valid_bst(root)
  between_val?(root, -Float::INFINITY, Float::INFINITY)
end

def between_val?(node, min, max)
  return true if node.nil?
  return false if node.val <= min || node.val >= max

  between_val?(node.left, min, node.val) && between_val?(node.right, node.val, max)
end
