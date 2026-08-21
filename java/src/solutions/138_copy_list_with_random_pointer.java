class Solution {
  public Node copyRandomList(Node head) {
    if (head == null) return null;

    var current = head;
    while (current != null) {
      var clone = new Node(current.val);
      clone.next = current.next;
      current.next = clone;
      current = clone.next;
    }

    current = head;
    while (current != null) {
      var clone = current.next;
      clone.random = current.random == null ? null : current.random.next;
      current = clone.next;
    }

    current = head;
    var copyHead = head.next;
    while (current != null) {
      var copyNext = current.next;
      current.next = copyNext.next;
      copyNext.next = current.next == null ? null : current.next.next;
      current = current.next;
    }

    return copyHead;
  }
}

class Node {
  int val;
  Node next;
  Node random;

  public Node(int val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}
