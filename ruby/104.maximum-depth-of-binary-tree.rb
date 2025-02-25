# @param {TreeNode} root
# @return {Integer}
def max_depth(root)
  depth(root, 0)
end

def depth(node, current_depth)
  return current_depth if node.nil?

  [depth(node.left, current_depth + 1), depth(node.right, current_depth + 1)].max
end
