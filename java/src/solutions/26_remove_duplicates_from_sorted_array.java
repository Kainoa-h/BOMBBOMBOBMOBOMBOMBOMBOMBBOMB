class Solution {
  public int removeDuplicates(int[] nums) {
    int writeIdx = 1;
    int prevNum = nums[0];
    for (int n : nums) {
      if (n != prevNum) {
        nums[writeIdx] = n;
        prevNum = n;
        writeIdx++;
      }
    }
    return writeIdx;
  }

  void main() {
    int[] nums = new int[] { 1, 1, 2 };
    int[] expectedNums = new int[] { 1, 2 };
    int res = removeDuplicates(nums);
    int ansLen = expectedNums.length;
    assert ansLen == res;
    for (int i = 0; i < ansLen; i++) {
      assert nums[i] == expectedNums[i];
    }
    System.out.println("test 1 pass");

    nums = new int[] { 0, 0, 1, 1, 1, 2, 2, 3, 3, 4 };
    expectedNums = new int[] { 0, 1, 2, 3, 4 };
    res = removeDuplicates(nums);
    ansLen = expectedNums.length;
    assert ansLen == res;
    for (int i = 0; i < ansLen; i++) {
      assert nums[i] == expectedNums[i];
    }
    System.out.println("test 2 pass");
  }
}
