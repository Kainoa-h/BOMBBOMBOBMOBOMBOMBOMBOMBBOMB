import java.util.HashMap;

class Solution {
  public int minimumHammingDistance(int[] source, int[] target, int[][] allowedSwaps) {

    var allowedSwapsDSU = new DSU(source.length);
    HashMap<Integer, HashMap<Integer, Integer>> availableValuesMap = new HashMap<>();
    int hammingDistance = 0;

    for (var swap : allowedSwaps) {
      allowedSwapsDSU.union(swap[0], swap[1]);
    }

    for (int i = 0; i < source.length; i++) {
      var val = source[i];
      var root = allowedSwapsDSU.find(i);
      availableValuesMap
          .computeIfAbsent(root, (k) -> new HashMap<>())
          .merge(val, 1, Integer::sum);
    }

    for (int i = 0; i < target.length; i++) {
      var val = target[i];
      var root = allowedSwapsDSU.find(i);
      var availableValues = availableValuesMap.get(root);
      if (availableValues.containsKey(val) && availableValues.get(val) > 0)
        availableValues.merge(val, 1, Math::subtractExact);
      else
        hammingDistance++;
    }

    return hammingDistance;
  }

  static class DSU {
    private int[] dsu;

    public DSU(int len) {
      dsu = new int[len];
      for (int i = 0; i < len; i++) {
        dsu[i] = i;
      }
    }

    public int find(int idx) {
      if (dsu[idx] == idx)
        return idx;
      return dsu[idx] = find(dsu[idx]);
    }

    public void union(int x, int y) {
      dsu[find(y)] = find(x);
    }
  }

  void main() {
    int x;
    x = minimumHammingDistance(new int[] { 1, 2, 3, 4 }, new int[] { 2, 1, 4, 5 }, new int[][] { { 0, 1 }, { 2, 3 } });
    System.out.println("out: " + x);
    assert x == 1;

    x = minimumHammingDistance(new int[] { 1, 2, 3, 4 }, new int[] { 1, 3, 2, 4 }, new int[][] {});
    System.out.println("out: " + x);
    assert x == 2;

    x = minimumHammingDistance(new int[] { 5, 1, 2, 4, 3 }, new int[] { 1, 5, 4, 2, 3 },
        new int[][] { { 0, 4 }, { 4, 2 }, { 1, 3 }, { 1, 4 } });
    System.out.println("out: " + x);
    assert x == 0;
  }
}
