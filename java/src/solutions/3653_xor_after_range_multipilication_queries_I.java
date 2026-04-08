class Solution {
  public int xorAfterQueries(int[] nums, int[][] queries) {
    for (int[] query : queries) {
      for (int idx = query[0]; idx <= query[1]; idx += query[2]) {
        nums[idx] = (int) (((long) nums[idx] * query[3]) % 1_000_000_007);
      }
    }

    int result = nums[0];
    for (int i = 1; i < nums.length; i++) {
      result ^= nums[i];
    }
    return result;
  }
}
