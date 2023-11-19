def add_two_numbers(l1, l2)
  head = ListNode.new(0)
  current = head
  carry = 0

  while l1 || l2 || carry.positive?
    sum = (l1&.val || 0) + (l2&.val || 0) + carry
    carry = sum / 10
    current.next = ListNode.new(sum % 10)
    current = current.next

    l1 = l1&.next
    l2 = l2&.next
  end

  head.next
end
