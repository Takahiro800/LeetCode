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

  def detectCycleFP(head: ListNode): Option[ListNode] = {
    @annotation.tailrec
    def findIntersection(slow: ListNode, fast: ListNode): Option[ListNode] = {
      (slow, fast, fast.next) match {
        case (_, null, _) | (_, _, null) => None
        case (s, f, _) if s == f         => Some(s)
        case (s, f, _) => findIntersection(s.next, f.next.next)
      }
    }

    @annotation.tailrec
    def findCycleStart(slow: ListNode, fast: ListNode): ListNode = {
      if (slow == fast) slow
      else findCycleStart(slow.next, fast.next)
    }

    findIntersection(head, head) match {
      case None               => None
      case Some(intersection) => Some(findCycleStart(head, intersection))
    }
  }
}
