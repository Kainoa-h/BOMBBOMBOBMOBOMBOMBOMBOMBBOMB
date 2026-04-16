class Solution {
  public boolean checkStrings(String s1, String s2) {
    if (s1.length() != s2.length())
      return false;

    int[] map = new int[26];
    for (int i = 0; i < s1.length(); i += 2) {
      map[s1.charAt(i) - 'a']++;
      map[s2.charAt(i) - 'a']--;
    }
    for (int x : map) {
      if (x != 0)
        return false;
    }

    for (int i = 1; i < s1.length(); i += 2) {
      map[s1.charAt(i) - 'a']++;
      map[s2.charAt(i) - 'a']--;
    }
    for (int x : map) {
      if (x != 0)
        return false;
    }
    return true;
  }

  void main() {
    assert checkStrings("abcd", "cdab");
    assert checkStrings("abcdba", "cabdab");
    assert !checkStrings("abcd", "dacb");
    assert !checkStrings("abe", "bea");
  }
}
