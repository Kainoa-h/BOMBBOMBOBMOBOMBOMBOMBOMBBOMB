class Solution {
  public int maximalSquare(char[][] matrix) {
    final int rows = matrix.length;
    final int cols = matrix[0].length;
    int[][] dpGrid = new int[rows + 1][cols + 1];

    var maxEdge = 0;
    for (int row = 0; row < rows; row++) {
      for (int col = 0; col < cols; col++) {
        var cell = matrix[row][col];
        if (cell == '0') {
          dpGrid[row + 1][col + 1] = 0;
          continue;
        }

        var topDP = dpGrid[row][col + 1];
        var leftDP = dpGrid[row + 1][col];
        var daigDP = dpGrid[row][col];

        var edge = Math.min(topDP, Math.min(leftDP, daigDP)) + 1;
        maxEdge = Math.max(maxEdge, edge);
        dpGrid[row + 1][col + 1] = edge;
      }
    }

    return maxEdge * maxEdge;
  }
}
