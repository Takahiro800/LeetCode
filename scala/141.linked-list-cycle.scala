import scala.annotation.tailrec

class ListNode(var _x: Int = 0) {
  var next: ListNode = null
  var x: Int = _x
}

object Solution {
  def hasCycle(head: ListNode): Boolean = {

    @tailrec
    def recursion(slow: ListNode, fast: ListNode): Boolean = {
      (Option(slow), Option(fast)) match {
        case (Some(s), Some(f)) if s.equals(f) => true;
        case (Some(s), Some(f)) if (Option(f.next).isDefined) =>
          recursion(s.next, f.next.next);
        case _ => false
      }
    }

    if (Option(head).isDefined) recursion(head, head.next) else false
  }
}
