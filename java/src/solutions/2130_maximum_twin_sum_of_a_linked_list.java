class Solution {
  public int pairSum(ListNode head) {
    var fast_head = head;
    var slow_head = head;
    while (fast_head != null) {
      fast_head = fast_head.next.next;
      slow_head = slow_head.next;
    }

    ListNode prev_head = null;
    var flipper_head = slow_head;
    while (flipper_head != null) {
      var next = flipper_head.next;
      flipper_head.next = prev_head;
      prev_head = flipper_head;
      flipper_head = next;
    }

    var end_head = prev_head;
    var start_head = head;
    int maxSum = 0;
    while (end_head != null) {
      maxSum = Math.max(maxSum, start_head.val + end_head.val);
      end_head = end_head.next;
      start_head = start_head.next;
    }

    return maxSum;
  }
}

class ListNode {
  int val;
  ListNode next;

  ListNode() {
  }

  ListNode(int val) {
    this.val = val;
  }

  ListNode(int val, ListNode next) {
    this.val = val;
    this.next = next;
  }
}
