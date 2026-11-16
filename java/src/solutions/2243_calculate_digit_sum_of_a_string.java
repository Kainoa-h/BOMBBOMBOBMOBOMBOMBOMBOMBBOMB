class Solution {
  public String digitSum(String s, int k) {
    if (s.length() <= k) {
      return s;
    }
    StringBuilder output = new StringBuilder();
    for (int i = 0; i < s.length(); i += k) {
      var segmentSum = 0;
      for (int c = i; c < Math.min(i + k, s.length()); c++) {
        segmentSum += s.charAt(c) - '0';
      }
      output.append(segmentSum);
    }

    return this.digitSum(output.toString(), k);
  }
}
