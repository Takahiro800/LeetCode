def hasCycle(head)
  slow = head
  fast = head

  while fast&.next&.next
    slow = slow.next
    fast = fast.next.next

    return true if slow == fast
  end

  false
end
