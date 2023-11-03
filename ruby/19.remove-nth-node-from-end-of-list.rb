# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer} n
# @return {ListNode}
def remove_nth_from_end(head, n)
  end_node = reverse_list(head)
  removed_head = remove_nth(end_node, n)

  reverse_list(removed_head)
end

def reverse_list(head)
  prev = nil
  current = head

  while current
    tmp = current.next
    current.next = prev
    prev = current
    current = tmp
  end

  prev
end

def remove_nth(head, n)
  return head.next if n == 1

  num = 1
  current = head

  while num < n - 1 && current
    current = current.next
    num += 1
  end

  current.next = current.next&.next
  head
end
