class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {TreeNode} root1
# @param {TreeNode} root2
# @return {TreeNode}
def merge_trees(root1, root2)
  return nil if root1.nil? && root2.nil?

  root = TreeNode.new(root1&.val.to_i + root2&.val.to_i)
  root.left = merge_trees(root1&.left, root2&.left)
  root.right = merge_trees(root1&.right, root2&.right)

  root
end
