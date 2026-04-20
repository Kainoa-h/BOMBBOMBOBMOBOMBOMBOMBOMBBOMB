class Solution {
  public int maxDistance(int[] colors) {
    int left = colors[0], right = colors[colors.length - 1];

    for (int i = 0; i < colors.length; i++) {
      if (left != colors[colors.length - 1 - i]) {
        return colors.length - 1 - i;
      }

      if (right != colors[i]) {
        return colors.length - 1 - i;
      }
    }
    return -1;
  }

  void main() {
    assert maxDistance(new int[] { 1, 1, 1, 6, 1, 1, 1 }) == 3;
    assert maxDistance(new int[] { 1, 8, 3, 8, 3 }) == 4;
    assert maxDistance(new int[] { 0, 1 }) == 1;
  }
}
