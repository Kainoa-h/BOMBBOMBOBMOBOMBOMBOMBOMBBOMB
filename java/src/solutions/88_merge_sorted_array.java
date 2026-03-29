import java.util.Arrays;

class Solution {
  public void merge(int[] nums1, int m, int[] nums2, int n) {
    int n1Idx = m - 1;
    int n2Idx = n - 1;
    int mergeIdx = m + n - 1;
    while (n2Idx >= 0) {
      if (n1Idx > -1 && nums1[n1Idx] > nums2[n2Idx]) {
        nums1[mergeIdx] = nums1[n1Idx];
        mergeIdx--;
        n1Idx--;
      } else {
        nums1[mergeIdx] = nums2[n2Idx];
        mergeIdx--;
        n2Idx--;
      }
    }
  }

  void main() {
    int[] n1 = new int[] { 1, 2, 3, 0, 0, 0 };
    int[] n2 = new int[] { 2, 5, 6 };
    merge(n1, 3, n2, 3);
    assert Arrays.equals(n1, new int[] { 1, 2, 2, 3, 5, 6 });

    n1 = new int[] { 0 };
    n2 = new int[] { 1 };
    merge(n1, 0, n2, 1);
    assert Arrays.equals(n1, new int[] { 1 });
  }
}
