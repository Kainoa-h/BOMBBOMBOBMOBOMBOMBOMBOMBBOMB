import java.util.Arrays;

class Solution {
  public int removeElement(int[] nums, int val) {
    int result = nums.length;
    int i = 0;
    int back = nums.length - 1;
    while (i <= back) {
      if (nums[i] == val) {
        nums[i] = nums[back];
        back--;
        result--;
        continue;
      }
      i++;
    }
    return result;
  }

  void main() {
    int[] nums = new int[] { 3, 2, 2, 3 };
    int[] expectedNums = new int[] { 2, 2 };
    int res = removeElement(nums, 3);
    int ansLen = 2;
    assert ansLen == res;
    Arrays.sort(nums, 0, ansLen);
    for (int i = 0; i < ansLen; i++) {
      assert nums[i] == expectedNums[i];
    }
    System.out.println("test 1 pass");

    nums = new int[] { 1 };
    expectedNums = new int[] {};
    res = removeElement(nums, 1);
    ansLen = 0;
    assert ansLen == res;
    Arrays.sort(nums, 0, ansLen);
    for (int i = 0; i < ansLen; i++) {
      assert nums[i] == expectedNums[i];
    }
    System.out.println("test 2 pass");
  }
}
