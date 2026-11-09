class Solution {
  public int minimumDeletions(String s) {
    int bSeen = 0;
    int moves = 0;
    for (int i = 0; i < s.length(); i++) {
      if (s.charAt(i) == 'b') {
        bSeen++;
      } else {
        moves = Math.min(bSeen, moves + 1);
      }
    }

    return moves;
  }
}

