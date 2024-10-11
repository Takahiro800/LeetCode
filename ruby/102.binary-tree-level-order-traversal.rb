class TreeNode
  attr_accessor :val, :left, :right

  def initialize(val = 0, left = nil, right = nil)
    @val = val
    @left = left
    @right = right
  end
end

# @param {TreeNode} root
# @return {Integer[][]}
def level_order(root)
  depth = max_depth(root)
  ans = Array.new(depth) { [] }

  level_order_recursion(root, ans, 0)
end

private

def level_order_recursion(node, array, depth)
  return array if node.nil?

  array[depth] << node.val
  level_order_recursion(node.left, array, depth + 1)
  level_order_recursion(node.right, array, depth + 1)
end

def max_depth(root)
  max_depth_recursion(root, 0)
end

def max_depth_recursion(node, current_depth)
  return current_depth if node.nil?

  current_depth += 1
  left_depth = max_depth_recursion(node.left, current_depth)
  right_depth = max_depth_recursion(node.right, current_depth)

  [left_depth, right_depth].max
end
