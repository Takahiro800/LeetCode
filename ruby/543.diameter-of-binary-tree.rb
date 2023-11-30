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
def diameter_of_binary_tree(root)
  @max = 0
  diameter(root)

  @max
end


def diameter(node)
  return -1 if node.nil?

  right = diameter(node.right) + 1
  left = diameter(node.left) + 1
  local_max = right + left

  # 後置if の方がmaxよりも処理が早い
  @max = local_max if local_max > @max

  # 三項演算子 の方がmaxよりも処理が早い
  left > right ? left : right
end
