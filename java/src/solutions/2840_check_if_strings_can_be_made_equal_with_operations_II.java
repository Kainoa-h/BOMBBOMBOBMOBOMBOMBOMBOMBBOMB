import java.util.Arrays;

class Solution {
  public boolean checkStrings(String s1, String s2) {
    if (s1.length() != s2.length())
      return false;

    int evenX = 0, evenD = 0, oddX = 0, oddD = 0;
    for (int i = 0; i < s1.length(); i++) {
      int c1 = s1.charAt(i);
      int c2 = s2.charAt(i);
      int diffX = c1 ^ c2;
      int diffD = (c1 * c1) - (c2 * c2);
      if (i % 2 == 0) {
        evenX ^= diffX;
        evenD += diffD;
      } else {
        oddX ^= diffX;
        oddD += diffD;
      }
    }
    return (evenX | oddX | evenD | oddD) == 0;
  }

  void main() {
    assert checkStrings("abcd", "cdab");
    assert checkStrings("abcdba", "cabdab");
    assert !checkStrings("abcd", "dacb");
    assert !checkStrings("abe", "bea");
  }
}
