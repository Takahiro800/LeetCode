package leetcode.`142`
class ListNode(var _x: Int = 0) {
  var next: ListNode = null
  var x: Int = _x
}

object Solution {
  def detectCycle(head: ListNode): ListNode = {
    if (head == null || head.next == null) return null

    var slow = head
    var fast = head

    while (fast != null && fast.next != null) {
      slow = slow.next
      fast = fast.next.next

      if (slow == fast) {
        fast = head
        while (slow != fast) {
          slow = slow.next
          fast = fast.next
        }

        return slow
      }
    }

    null
  }
}
