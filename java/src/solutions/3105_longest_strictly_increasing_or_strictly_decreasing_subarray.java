class Solution {
  public int longestMonotonicSubarray(int[] nums) {
    var longestRun = 1;
    int incCurrentRun = 1, decCurrentRun = 1;
    for (int i = 1; i < nums.length; i++) {
      incCurrentRun =  (nums[i-1] < nums[i]) ? incCurrentRun + 1 : 1;
      decCurrentRun =  (nums[i-1] > nums[i]) ? decCurrentRun + 1: 1;
      longestRun = Math.max(longestRun, Math.max(incCurrentRun, decCurrentRun));
    }
    return longestRun;
  }
}
