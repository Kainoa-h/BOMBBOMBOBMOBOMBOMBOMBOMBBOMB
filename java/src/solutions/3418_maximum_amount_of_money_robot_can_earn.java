class Solution {
  public int maximumAmount(int[][] coins) {
    final int NEUTRALISES = 2;
    Integer[][] rollingRow = new Integer[coins[0].length][NEUTRALISES + 1];
    rollingRow[0][0] = 0;

    for (int rowIdx = 0; rowIdx < coins.length; rowIdx++) {
      for (int colIdx = 0; colIdx < coins[0].length; colIdx++) {
        int coin = coins[rowIdx][colIdx];
        Integer[] rollingCol = rollingRow[colIdx];

        for (int i = 0; i < NEUTRALISES + 1; i++) {
          Integer left = null;
          if (colIdx != 0) {
            left = rollingRow[colIdx - 1][i];
          }
          Integer top = rollingCol[i];
          if (left == null && top == null) {
            continue;
          } else if (left == null) {
            rollingCol[i] = top + coin;
          } else if (top == null) {
            rollingCol[i] = left + coin;
          } else {
            rollingCol[i] = Integer.max(top, left) + coin;
          }
        }

        if (coin < 0) {
          var snapshot = rollingCol.clone();
          for (int i = 0; i < NEUTRALISES; i++) {
            if (snapshot[i] == null)
              break;
            int neuralisedVal = snapshot[i] - coin;
            if (rollingCol[i + 1] == null || neuralisedVal > rollingCol[i + 1])
              rollingCol[i + 1] = neuralisedVal;
          }
        }
      }
    }

    var last = rollingRow[rollingRow.length - 1];
    int result = last[0];
    for (Integer i : last) {
      if (i == null) {
        continue;
      }
      if (i > result) {
        result = i;
      }
    }
    return result;

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
