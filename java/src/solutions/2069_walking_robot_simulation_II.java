class Robot {
  int[] xy = new int[] { 0, 0 };
  final int[] widthHeight;
  final int[][] dirVec = new int[][] { { 1, 0 }, { 0, 1 }, { -1, 0 }, { 0, -1 } };
  final String[] dirStr = new String[] { "East", "North", "West", "South" };
  int dirIdx = 0;

  public Robot(int width, int height) {
    widthHeight = new int[] { width - 1, height - 1 };
  }

  public void step(int num) {
    int idx = dirIdx & 1;
    int z = xy[idx];
    int dir = dirVec[dirIdx][idx];
    int space = widthHeight[idx];
    int nextZ = (dir * num) + z;
    if (nextZ < 0) {
      xy[idx] = 0;
      dirIdx = (dirIdx + 1) % 4;
      step(Math.abs(nextZ));
    } else if (nextZ > space) {
      xy[idx] = space;
      dirIdx = (dirIdx + 1) % 4;
      step(nextZ - space);
    } else {
      xy[idx] = nextZ;
    }
  }

  public int[] getPos() {
    return xy;
  }

  public String getDir() {
    return dirStr[dirIdx];
  }
}
