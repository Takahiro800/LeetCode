# frozen_string_literal: true

class ListNode
  attr_accessor :val, :next

  def initialize(val = 0, _next = nil)
    @val = val
    @next = _next
  end
end

# @param {ListNode} head
# @return {ListNode}
def delete_duplicates(head)
  new_head = ListNode.new(nil, head)
  prev = new_head
  target = head

  while target&.next
    if target.val == target.next.val
      duplicate_node = skip_duplicates(target)
      target = duplicate_node
      prev.next = duplicate_node.next
    else
      prev = target
    end

    target = target.next
  end

  new_head.next
end

def skip_duplicates(node)
  duplicate_node = node
  while duplicate_node&.next&.val == duplicate_node.val
    duplicate_node = duplicate_node.next
  end
  duplicate_node
end
