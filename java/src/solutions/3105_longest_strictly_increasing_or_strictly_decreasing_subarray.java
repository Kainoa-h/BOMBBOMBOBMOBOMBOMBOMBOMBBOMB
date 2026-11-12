class Solution {
  public int longestMonotonicSubarray(int[] nums) {
    var LongestRun = 1;
    var incCurrentRun = 1;
    var decCurrentRun = 1;
    for (int i = 1; i < nums.length; i++) {
      if (nums[i-1] < nums[i]) {
        incCurrentRun++;
      } else {
        LongestRun = Math.max(LongestRun, incCurrentRun);
        incCurrentRun = 1;
      }
      
      if (nums[i-1] > nums[i]) {
        decCurrentRun++;
      } else {
        LongestRun = Math.max(LongestRun, decCurrentRun);
        decCurrentRun = 1;
      }
    }
    return Math.max(LongestRun, Math.max(incCurrentRun, decCurrentRun));
  }
}
