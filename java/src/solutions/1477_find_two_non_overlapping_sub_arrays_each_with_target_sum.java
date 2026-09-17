import java.util.Arrays;

class Solution {
  public int minSumOfLengths(int[] arr, int target) {
    var n = arr.length;
    var dp = new int[n + 1];
    Arrays.fill(dp, n + 1);
    int result = n + 1, left = 0, count = 0;
    for (var right = 0; right < n; right++) {
      count += arr[right];
      dp[right + 1] = dp[right];

      while (count > target) {
        count -= arr[left];
        left++;
      }

      if (count == target) {
        var len = right - left + 1;
        dp[right + 1] = Math.min(dp[right], len);
        result = Math.min(result, dp[left] + len);
      }
    }

    return result == n + 1 ? -1 : result;
  }
}
