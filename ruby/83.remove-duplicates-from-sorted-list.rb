# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {ListNode}
def delete_duplicates(head)
  current_node = next_node = head

  while next_node
    while current_node.val == next_node&.val
      next_node = next_node.next
    end

    current_node.next = next_node
    current_node = next_node
    next_node = next_node&.next
  end

  head
end
