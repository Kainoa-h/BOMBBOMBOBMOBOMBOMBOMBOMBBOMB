import java.util.*;

class Solution {
  public int minimumSumSubarray(List<Integer> nums, int l, int r) {
    final int[] prefix = new int[nums.size() + 1];
    int sum = 0;
    for (int i = 0; i < nums.size(); i++) {
      var x = nums.get(i);
      prefix[i] = sum;
      sum += x;
    }
    prefix[nums.size()] = sum;

    int min = Integer.MAX_VALUE;
    for (int startIdx = 0; startIdx < prefix.length - l + 1; startIdx++) {
      for (int endIdx = startIdx + l; endIdx <= startIdx + r && endIdx < prefix.length; endIdx++) {
        var subArraySum = prefix[endIdx] - prefix[startIdx];
        if (subArraySum > 0) {
          min = Math.min(min, subArraySum);
        }
      }
    }
    return min == Integer.MAX_VALUE ? -1 : min;
  }
}
