import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Stack;
import java.util.stream.Collectors;

class Solution {
  public List<Integer> survivedRobotsHealths(int[] positions, int[] healths, String directions) {
    int n = positions.length;
    Integer[] indices = new Integer[n];
    for (int i = 0; i < n; i++) {
      indices[i] = i;
    }
    Arrays.sort(indices, (a, b) -> Integer.compare(positions[a], positions[b]));

    Stack<Integer> stack = new Stack<>();
    for (int i : indices) {
      if (directions.charAt(i) == 'R') {
        stack.push(i);
        continue;
      }

      while (stack.size() != 0 && healths[i] > 0) {
        int mvRightIdx = stack.pop();
        if (healths[mvRightIdx] > healths[i]) {
          healths[mvRightIdx]--;
          healths[i] = 0;
          stack.push(mvRightIdx);
        } else if (healths[mvRightIdx] < healths[i]) {
          healths[i]--;
          healths[mvRightIdx] = 0;
        } else {
          healths[mvRightIdx] = 0;
          healths[i] = 0;
        }
      }
    }

    return Arrays.stream(healths).filter((x) -> x > 0).boxed()
        .collect(Collectors.toCollection(ArrayList::new));
  }

  void main() {
    System.out.println("-");
    assert List.of(2, 17, 9, 15, 10).equals(
        survivedRobotsHealths(new int[] { 5, 4, 3, 2, 1 }, new int[] { 2, 17, 9, 15, 10 }, "RRRRR"));

    System.out.println("-");
    assert List.of(14).equals(
        survivedRobotsHealths(new int[] { 3, 5, 2, 6 }, new int[] { 10, 10, 15, 12 }, "RLRL"));

    System.out.println("-");
    assert survivedRobotsHealths(new int[] { 1, 2, 5, 6 }, new int[] { 10, 10, 11, 11 }, "RLRL").size() == 0;

    System.out.println("-");
    assert List.of(47).equals(
        survivedRobotsHealths(new int[] { 12, 33, 37 }, new int[] { 49, 5, 38 }, "RLL"));
  }
}
