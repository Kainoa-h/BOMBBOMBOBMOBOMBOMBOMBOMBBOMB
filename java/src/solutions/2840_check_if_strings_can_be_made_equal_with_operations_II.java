import java.util.Arrays;

class Solution {
  public boolean checkStrings(String s1, String s2) {
    if (s1.length() != s2.length())
      return false;

    int[] even1 = new int[26];
    int[] even2 = new int[26];
    int[] odd1 = new int[26];
    int[] odd2 = new int[26];
    for (int i = 0; i < s1.length(); i++) {
      if (i % 2 == 0) {
        even1[s1.charAt(i) - 'a'] += 1;
        even2[s2.charAt(i) - 'a'] += 1;
      } else {

        odd1[s1.charAt(i) - 'a'] += 1;
        odd2[s2.charAt(i) - 'a'] += 1;
      }
    }
    return Arrays.equals(even1, even2) && Arrays.equals(odd1, odd2);
  }

  void main() {
    assert checkStrings("abcd", "cdab");
    assert checkStrings("abcdba", "cabdab");
    assert !checkStrings("abcd", "dacb");
    assert !checkStrings("abe", "bea");
  }
}
