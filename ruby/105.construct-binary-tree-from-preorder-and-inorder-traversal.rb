class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {Integer[]} preorder
# @param {Integer[]} inorder
# @return {TreeNode}
def build_tree(preorder, inorder)
  return if inorder.empty?

  root_val = preorder.shift
  root_index = inorder.index(root_val)

  root = TreeNode.new(root_val)
  root.left = build_tree(preorder, inorder[...root_index])
  root.right = build_tree(preorder, inorder[root_index + 1..])

  root
end
