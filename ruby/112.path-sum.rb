class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {TreeNode} root
# @param {Integer} target_sum
# @return {Boolean}
def has_path_sum(root, target_sum)
  dfs?(root, target_sum)
end

def dfs?(node, target_sum)
  return false if node.nil?
  return target_sum == node.val if node.left.nil? && node.right.nil?

  dfs?(node.left, target_sum - node.val) || dfs?(node.right, target_sum - node.val)
end
