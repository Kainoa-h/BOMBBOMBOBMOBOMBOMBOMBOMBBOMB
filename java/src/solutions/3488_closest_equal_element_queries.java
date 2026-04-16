/*
Input: nums = [1,3,1,4,1,3,2], queries = [0,3,5]

Output: [2,-1,3]

Explanation:

Query 0: The element at queries[0] = 0 is nums[0] = 1. The nearest index with the same value is 2, and the distance between them is 2.
Query 1: The element at queries[1] = 3 is nums[3] = 4. No other index contains 4, so the result is -1.
Query 2: The element at queries[2] = 5 is nums[5] = 3. The nearest index with the same value is 1, and the distance between them is 3 (following the circular path: 5 -> 6 -> 0 -> 1).
 * */

import java.util.List;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

class Solution {
  public List<Integer> solveQueries(int[] nums, int[] queries) {
    Map<Integer, List<Integer>> map = new HashMap<>();
    List<Integer> result = new ArrayList<>();

    for (int idx = 0; idx < nums.length; idx++) {
      var val = nums[idx];
      map.merge(val, new ArrayList<>(List.of(idx)), (a, b) -> {
        a.addAll(b);
        return a;
      });
    }

    for (int idx : queries) {
      var val = nums[idx];
      var idxArr = map.get(val);
      if (idxArr.size() == 1) {
        result.add(-1);
        continue;
      }

      int i = Collections.binarySearch(idxArr, idx);
      int left = i - 1, right = i + 1;
      int left_diff, right_diff;
      if (i == 0) {
        left_diff = nums.length - idxArr.getLast() + idx;
        right_diff = Math.abs(idxArr.get(right) - idx);
      } else if (i == idxArr.size() - 1) {
        left_diff = Math.abs(idxArr.get(left) - idx);
        right_diff = nums.length - idx + idxArr.getFirst();
      } else {
        left_diff = Math.abs(idxArr.get(left) - idx);
        right_diff = Math.abs(idxArr.get(right) - idx);
      }
      // System.out.println("val:" + val);
      // System.out.println("idxArr:" + idxArr.toString());
      // System.out.println("left_idx:" + left);
      // System.out.println("i:" + i);
      // System.out.println("right_idx:" + right);

      result.add(
          Math.min(left_diff, right_diff));

    }

    return result;
  }

  void main() {
    System.out.println(
        solveQueries(new int[] { 1, 3, 1, 4, 1, 3, 2 }, new int[] { 0, 3, 5 }).toString());
  }
}
