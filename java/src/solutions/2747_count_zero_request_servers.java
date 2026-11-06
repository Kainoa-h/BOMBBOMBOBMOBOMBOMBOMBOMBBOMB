import java.util.*;

class Solution {
  public int[] countServers(int n, int[][] logs, int x, int[] queries) {
    Arrays.sort(logs, (a, b) -> Integer.compare(a[1], b[1]));
    Integer[] sortedQueriesIndexes = new Integer[queries.length];
    for (int i = 0; i < queries.length; i++) {
      sortedQueriesIndexes[i] = i;
    }
    Arrays.sort(sortedQueriesIndexes, (a, b) -> Integer.compare(queries[a], queries[b]));

    int[] queryResults = new int[queries.length];
    int pointer = 0;
    {
      int firstStart = queries[sortedQueriesIndexes[0]] - x;
      while (logs[pointer][1] < firstStart) {
        pointer++;
      }
    }
    var nextStartPointer = pointer;
    for (var i = 0; i < sortedQueriesIndexes.length; i++) {
      var qIdx = sortedQueriesIndexes[i];
      var end = queries[qIdx];
      var start = end - x;
      Integer nextStart = null;
      if (i < sortedQueriesIndexes.length - 1) {
        nextStart = queries[sortedQueriesIndexes[i + 1]] - x;
      }
      Set<Integer> set = new HashSet<>();
      while (pointer < logs.length && logs[pointer][1] >= start && logs[pointer][1] <= end) {
        set.add(logs[pointer][0]);
        if (nextStart != null && logs[pointer][1] < nextStart) {
          nextStartPointer++;
        }
        pointer++;
      }
      queryResults[qIdx] = n - set.size();
      pointer = nextStartPointer;
    }

    return queryResults;
  }

  void main() {
    Solution s = new Solution();
    var ans1 = s.countServers(3, new int[][] { new int[] { 1, 3 }, new int[] { 2, 6 }, new int[] { 1, 5 } }, 5,
        new int[] { 10, 11 });
  }
}
