import java.util.HashSet;
import java.util.Set;

class Solution {
  public int robotSim(int[] commands, int[][] obstacles) {
    Set<String> obstaclesSet = new HashSet<>();
    int max = 0;
    for (int[] obs : obstacles) {
      obstaclesSet.add(obs[0] + "," + obs[1]);
    }

    int x = 0, y = 0;
    int xDir = 0, yDir = 1;
    for (int cmd : commands) {
      if (cmd == -2) {
        int p = xDir;
        xDir = -yDir;
        yDir = p;
        continue;
      }
      if (cmd == -1) {
        int p = -xDir;
        xDir = yDir;
        yDir = p;
        continue;
      }

      int nextRowIdx = x, nextColIdx = y;
      for (int i = 0; i < cmd; i++) {
        nextRowIdx += xDir;
        nextColIdx += yDir;
        if (obstaclesSet.contains(nextRowIdx + "," + nextColIdx)) {
          nextRowIdx -= xDir;
          nextColIdx -= yDir;
          break;
        }

      }
      x = nextRowIdx;
      y = nextColIdx;
      max = Math.max(max, (int) (Math.pow(x, 2) + Math.pow(y, 2)));
    }
    return max;
  }

  void main() {
    assert 25 == robotSim(new int[] { 4, -1, 3 }, new int[][] {});
    assert 65 == robotSim(new int[] { 4, -1, 4, -2, 4 }, new int[][] { { 2, 4 } });
    assert 36 == robotSim(new int[] { 6, -1, -1, 6 }, new int[][] { { 0, 0 } });
  }
}
