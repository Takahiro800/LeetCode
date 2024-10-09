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
  if root.nil?
    return false
  end

  if roof?(root)
    return target_sum == root.val
  end

  has_path_sum(root.left, target_sum - root.val) || has_path_sum(root.right, target_sum - root.val)
end

def roof?(node)
  node.left.nil? && node.right.nil?
end
