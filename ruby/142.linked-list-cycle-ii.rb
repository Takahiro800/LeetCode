def detectCycle(head)
  slow = head
  fast = head

  while fast&.next&.next
    slow = slow.next
    fast = fast.next.next

    if slow == fast
      break
    end
  end

  return unless fast&.next&.next

  fast = head

  while slow != fast
    slow = slow.next
    fast = fast.next
  end

  slow
end
