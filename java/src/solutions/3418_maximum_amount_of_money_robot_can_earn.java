import java.util.Arrays;

class Solution {
  public int maximumAmount(int[][] coins) {
    int[][] dp = new int[coins[0].length + 1][3];
    for (var col : dp) {
      Arrays.fill(col, Integer.MIN_VALUE / 2);
    }
    dp[1][0] = 0;
    dp[1][1] = 0;
    dp[1][2] = 0;

    for (var row : coins) {
      for (int colIdx = 1; colIdx < dp.length; colIdx++) {
        final int coin = row[colIdx - 1];
        dp[colIdx][2] = Math.max(
            Math.max(dp[colIdx][2], dp[colIdx - 1][2]) + coin,
            Math.max(dp[colIdx][1], dp[colIdx - 1][1]));
        dp[colIdx][1] = Math.max(
            Math.max(dp[colIdx][1], dp[colIdx - 1][1]) + coin,
            Math.max(dp[colIdx][0], dp[colIdx - 1][0]));
        dp[colIdx][0] = Math.max(dp[colIdx][0], dp[colIdx - 1][0]) + coin;
      }
    }

    return dp[dp.length - 1][2];
  }

  void main() {
    int x = 0;

    x = maximumAmount(new int[][] { { 0, 1, -1 }, { 1, -2, 3 }, { 2, -3, 4 } });
    System.out.println(x);
    assert x == 8;

    x = maximumAmount(new int[][] { { 10, 10, 10 }, { 10, 10, 10 } });
    System.out.println(x);
    assert x == 40;

    x = maximumAmount(new int[][] { { -7, 12, 12, 13 }, { -6, 19, 19, -6 }, { 9, -2, -10, 16 }, { -4, 14, -10, -9 } });
    System.out.println(x);
    assert x == 60;
  }
}
