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
  left = head
  right = left

  n.times.each { |_| right = right.next }
  return left.next unless right

  while right&.next
    left = left.next
    right = right.next
  end

  left.next = left.next&.next
  head
end
