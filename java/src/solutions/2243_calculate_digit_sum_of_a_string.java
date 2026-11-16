class Solution {
  public String digitSum(String s, final int k) {
    StringBuilder sb = new StringBuilder();
    while (s.length() > k) {
      for (int i = 0; i < s.length(); i += k) {
        var segmentSum = 0;
        for (int c = i; c < Math.min(i + k, s.length()); c++) {
          segmentSum += s.charAt(c) - '0';
        }
        sb.append(segmentSum);
      }
      s = sb.toString();
      sb.setLength(0);
    }

    return s;
  }
}
