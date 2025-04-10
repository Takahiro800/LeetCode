package leetcode.`82`

class ListNode(_x: Int = 0, _next: ListNode = null) {
  var next: ListNode = _next
  var x: Int = _x
}

object Solution {
  def deleteDuplicates(head: ListNode): ListNode = {
    @annotation.tailrec
    def removeDuplicates(prev: ListNode, curr: ListNode): ListNode = {
      if (curr == null) {
        prev.next = null
        prev.next
      } else if (curr.next != null && curr.x == curr.next.x) {
        val nextDistinct = findNextDistinct(curr)
        removeDuplicates(prev, nextDistinct)
      } else {
        prev.next = curr
        removeDuplicates(curr, curr.next)
      }
    }

    @annotation.tailrec
    def findNextDistinct(node: ListNode): ListNode = {
      if (node.next == null || node.x != node.next.x) node.next
      else findNextDistinct(node.next)
    }

    val dummy = new ListNode(0)
    removeDuplicates(dummy, head)
    dummy.next
  }
}
