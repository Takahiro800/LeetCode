package leetcode.`82`

class ListNode(_x: Int = 0, _next: ListNode = null) {
  var next: ListNode = _next
  var x: Int = _x
}

object Solution {
  def deleteDuplicates(head: ListNode): ListNode = {
    val dummy = new ListNode(0, head)
    var prevNode: ListNode = dummy
    var target: ListNode = head

    while (target != null) {
      if (target.next != null && target.x == target.next.x) {
        while (target.next != null && target.x == target.next.x) {
          target = target.next
        }
        prevNode.next = target.next
      } else {
        prevNode.next = target
        prevNode = target
      }
      target = target.next
    }

    dummy.next
  }
}
