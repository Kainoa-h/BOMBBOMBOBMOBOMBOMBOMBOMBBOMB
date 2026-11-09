class Solution {
  public int minimumDeletions(String s) {
    int removeAs = 0;
    boolean seenB = false;
    for (int i = 0; i < s.length(); i++) {
      seenB = s.charAt(i) == 'b' || seenB;

      if (seenB && s.charAt(i) == 'a') {
        removeAs++;
      }
    }

    int removeBs = 0;
    boolean seenA = false;
    for (int i = s.length() -1; i >= 0; i--) {
      seenA = s.charAt(i) == 'a' || seenA;

      if (seenA && s.charAt(i) == 'b') {
        removeBs++;
      }
    }

    return Math.min(removeAs, removeBs);
  }
}
