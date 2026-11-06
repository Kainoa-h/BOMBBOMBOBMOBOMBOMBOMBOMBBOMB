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
    int[] serverBucketCount = new int[n + 1];
    int serversInWindow = 0;
    var left = 0;
    var right = 0;

    for (var qIdx: sortedQueriesIndexes) {
      var end = queries[qIdx];
      var start = end - x;

      while (right < logs.length && logs[right][1] <= end) {
        var server = logs[right][0];
        serverBucketCount[server]++;
        if (serverBucketCount[server] == 1) {
          serversInWindow++;
        }
        right++;
      }

      while (left < logs.length && logs[left][1] < start) {
        var server = logs[left][0];
        serverBucketCount[server]--;
        if (serverBucketCount[server] == 0) {
          serversInWindow--;
        }
        left++;
      }

      queryResults[qIdx] = n - serversInWindow;
    }

    return queryResults;
  }

  void main() {
    Solution s = new Solution();
    var ans1 = s.countServers(3, new int[][] { new int[] { 1, 3 }, new int[] { 2, 6 }, new int[] { 1, 5 } }, 5,
        new int[] { 10, 11 });
  }
}
