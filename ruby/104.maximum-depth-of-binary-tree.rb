# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @return {Integer}
def max_depth(root)
  # return 0 if root.nil?

  depth(root, 0)
end

def depth(node, current_depth)
  return current_depth if node.nil?

  current_depth += 1
  [depth(node.left, current_depth), depth(node.right, current_depth)].max
end
