def add_two_numbers(l1, l2)
  # 必須。headを用意する
  result_node = ListNode.new
  head = result_node
  carry = 0

  while l1 && l2
    current_sum = l1.val + l2.val + carry
    result_node.val = current_sum % 10
    carry = current_sum / 10

    l1 = l1.next
    l2 = l2.next

    if l1 || l2 || carry.positive?
      result_node.next = ListNode.new
      result_node = result_node.next
    end
  end

  while l1
    current_sum = l1.val + carry
    result_node.val = current_sum % 10
    carry = current_sum / 10

    l1 = l1.next

    if l1 || carry.positive?
      result_node.next = ListNode.new
      result_node = result_node.next
    end
  end

  while l2
    current_sum = l2.val + carry
    result_node.val = current_sum % 10
    carry = current_sum / 10

    l2 = l2.next

    if l2 || carry.positive?
      result_node.next = ListNode.new
      result_node = result_node.next
    end
  end

  result_node.val = carry if carry.positive?

  head
end
