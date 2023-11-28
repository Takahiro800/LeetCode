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
# @return {TreeNode}
def invert_tree(root)
    invert(root)

    root
end

private

def invert(node)
    return if node.nil?

    tmp_right = node.right
    node.right = node.left
    node.left = tmp_right

    invert(node.left)
    invert(node.right)
end
