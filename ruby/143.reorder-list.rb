# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {Void} Do not return anything, modify head in-place instead.
def reorder_list(head)
  # find center
  slow = head
  fast = head.next

  while fast && fast.next
    slow = slow.next
    fast = fast.next.next
  end

  # 後半をreverse
  second = slow.next
  prev = nil
  slow.next = nil

  while second
    tmp = second.next
    second.next = prev
    prev = second
    second = tmp
  end

  # 前半と後半をmerge
  first = head
  second = prev

  while second
    tmp1 = first.next
    tmp2 = second.next

    first.next = second
    second.next = tmp1

    first = tmp1
    second = tmp2
  end
end
