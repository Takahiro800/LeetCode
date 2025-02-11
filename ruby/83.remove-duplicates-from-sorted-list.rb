# frozen_string_literal: true

def delete_duplicates(head)
  current_node = head
  next_node = head

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
