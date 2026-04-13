class Solution {

  public int getMinDistance(int[] nums, int target, int start) {
    int min = Integer.MAX_VALUE;
    for (int i = 0; i < nums.length; i++) {
      if (target == nums[i]) {
        int local = Math.abs(i - start);
        if (local < min) {
          min = local;
        } else {
          break;
        }
      }
    }
    return min;
  }
}
