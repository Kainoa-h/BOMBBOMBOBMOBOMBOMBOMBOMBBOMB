class Solution {
  public int minimumDeletions(String s) {
    int aRight = 0;
    for (int i = 0; i < s.length(); i++) {
      if (s.charAt(i) == 'a') {
        aRight += 1;
      }
    }

    int bLeft = 0;
    int moves = aRight;
    for (int i = 0; i < s.length(); i++) {
      if (s.charAt(i) == 'a') {
        aRight--;
      } else {
        bLeft++;
      }
      moves = Math.min(moves, aRight + bLeft);
    }

    return moves;
  }
}

