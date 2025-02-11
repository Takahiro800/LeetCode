# frozen_string_literal: true

# @param {ListNode} head
# @return {ListNode}
def reverse_list(head)
  recursive_reverse_list(head)
end

def recursive_reverse_list(current, prev = nil)
  return prev unless current

  next_node = current.next
  current.next = prev
  recursive_reverse_list(next_node, current)
end
