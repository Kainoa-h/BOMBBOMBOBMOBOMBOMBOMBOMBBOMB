class Solution {
  public String largestPalindromic(String num) {
    int[] bucket = new int[10];
    for (int i = 0; i < num.length(); i++) {
      bucket[num.charAt(i) - '0']++;
    }
    
    StringBuilder left = new StringBuilder();
    for (int i = 9; i >= 0; i--) {
      var count = bucket[i];
      if (count > 1 && (i != 0 || left.length() != 0)) {
        var use = count / 2;
        left.append(String.valueOf(i).repeat(use));
        bucket[i] -= use * 2;
      }
    }

    String mid = "";
    for (int i = 9; i >= 0; i--) {
      var count = bucket[i];
      if (count > 0) {
        mid = String.valueOf(i);
        break;
      }
    }

    StringBuilder right = new StringBuilder(left);
    right.reverse();

    left.append(mid);
    left.append(right);

    return left.toString();
  }
}
