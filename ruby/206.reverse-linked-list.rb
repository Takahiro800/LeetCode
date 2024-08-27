class ListNode
  attr_accessor :val, :next

  def initialize(val = 0, _next = nil)
    @val = val
    @next = _next
  end
end

# @param {ListNode} head
# @return {ListNode}
def reverse_list(head)
  recursive_reverse_list(head)
end

def recursive_reverse_list(head, prev = nil)
  return prev unless head

  next_node = head.next
  head.next = prev
  recursive_reverse_list(next_node, head)
end
