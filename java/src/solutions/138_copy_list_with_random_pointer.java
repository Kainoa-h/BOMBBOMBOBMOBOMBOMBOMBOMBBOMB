import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

class Solution {
  static class Depo {
    public int index;
    public int value;
    public int hashcode;
    public int nextRandHashcode;
    public int nextRandIndex;

    public Depo(int index, int value, int hashcode, int nextRandHashcode, int nextRandIndex) {
      this.index = index;
      this.value = value;
      this.hashcode = hashcode;
      this.nextRandHashcode = nextRandHashcode;
      this.nextRandIndex = nextRandIndex;
    }
  }

  public Node copyRandomList(Node head) {
    if (head == null) {
      return null;
    }
    
    Map<Integer, Integer> map = new HashMap<>();
    List<Depo> depoList = new ArrayList<>();

    while (head != null) {
      map.put(head.hashCode(), depoList.size());
      depoList.add(new Depo(depoList.size(), head.val, head.hashCode(), head.random != null ? head.random.hashCode() : -1, -1));
      head = head.next;
    }

    List<Node> nodeList = new ArrayList<>();
    nodeList.add(new Node(-1));
    for (var x : depoList) {
      x.nextRandIndex = x.nextRandHashcode == -1 ? -1 : map.get(x.nextRandHashcode);
      var node = new Node(x.value);
      nodeList.getLast().next = node;
      nodeList.add(node);
    }

    for (int i = 0; i < depoList.size(); i++) {
      nodeList.get(i + 1).random = depoList.get(i).nextRandIndex == -1 ? null : nodeList.get(depoList.get(i).nextRandIndex + 1);
    }

    return nodeList.get(1);
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
