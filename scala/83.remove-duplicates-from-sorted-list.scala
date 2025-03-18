package leetcode.`83`

class ListNode(_x: Int = 0, _next: ListNode = null) {
  var next: ListNode = _next
  var x: Int = _x
}

object Solution {
  def deleteDuplicates(head: ListNode): ListNode = {
    var current = head

    while (current != null && current.next != null) {
      if (current.x == current.next.x) {
        current.next = current.next.next
      } else {
        current = current.next
      }
    }

    head
  }
}
