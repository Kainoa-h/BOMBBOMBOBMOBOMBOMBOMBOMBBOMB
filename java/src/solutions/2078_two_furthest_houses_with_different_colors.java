class Solution {
  public int maxDistance(int[] colors) {
    int left = colors[0], right = colors[colors.length - 1];
    int lastIdx = colors.length - 1;

    for (int dist = lastIdx; dist > 0; dist--) {
      if (left != colors[dist] || right != colors[lastIdx - dist]) {
        return dist;
      }
    }

    return 0;
  }

  void main() {
    assert maxDistance(new int[] { 1, 1, 1, 6, 1, 1, 1 }) == 3;
    assert maxDistance(new int[] { 1, 8, 3, 8, 3 }) == 4;
    assert maxDistance(new int[] { 0, 1 }) == 1;
  }
}
