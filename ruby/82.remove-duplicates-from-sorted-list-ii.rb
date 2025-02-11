# frozen_string_literal: true

# @param {ListNode} head
# @return {ListNode}
def delete_duplicates(head)
  new_head = ListNode.new(nil, head)
  prev_node = new_head
  target = head

  while target
    if target&.next&.val == target&.val
      while target&.next&.val == target&.val
        target = target.next
      end
    else
      prev_node.next = target
      prev_node = target
    end
    target = target.next
  end

  prev_node.next = target
  new_head.next
end
