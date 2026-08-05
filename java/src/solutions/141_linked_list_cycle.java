import java.util.HashSet;
import java.util.Set;

class ListNode {
  int val;
  ListNode next;

  ListNode(int x) {
    val = x;
    next = null;
  }
}

class Solution {
  public boolean hasCycle(ListNode head) {
    while (head != null) {
      if (head.val == Integer.MAX_VALUE){
        return true;
      }
      head.val = Integer.MAX_VALUE;
      head = head.next;
    }
    return false;
  }
}
