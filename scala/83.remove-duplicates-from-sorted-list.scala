package leetcode.`83`

class ListNode(_x: Int = 0, _next: ListNode = null) {
  var next: ListNode = _next
  var x: Int = _x
}

object Solution {
  def deleteDuplicates(head: ListNode): ListNode = {
    @annotation.tailrec
    def recursion(prev: ListNode, current: ListNode): ListNode = {
      if (current == null) head
      else if (prev != null && prev.x == current.x) {
        prev.next = current.next
        recursion(prev, current.next)
      } else {
        recursion(current, current.next)
      }
    }

    recursion(null, head)
  }
}
