import java.util.Arrays;

class Solution {
  public int coinChange(int[] coins, int amount) {
    if (amount == 0) {
      return 0;
    }
    int[] dp = new int[amount + 1];
    dp[0] = 1;
    for (var i = 1; i < dp.length; i++) {
      for (var coin : coins) {
        if (i < coin) {
          continue;
        }
        var start = i - coin;
        if (dp[start] != 0)
          if (dp[i] == 0)
            dp[i] = dp[start] + 1;
          else
            dp[i] = Math.min(dp[i], dp[start] + 1);
      }
    }
    return dp[amount] == 0 ? -1 : dp[amount] - 1;
  }
}
