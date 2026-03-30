import java.util.HashMap;

class Solution {
  public boolean checkStrings(String s1, String s2) {
    HashMap<Character, Long> map1Even = new HashMap<>();
    HashMap<Character, Long> map2Even = new HashMap<>();
    for (int i = 0; i < s1.length(); i += 2) {
      map1Even.merge(s1.charAt(i), 1L, Long::sum);
      map2Even.merge(s2.charAt(i), 1L, Long::sum);
    }
    HashMap<Character, Long> map1Odd = new HashMap<>();
    HashMap<Character, Long> map2Odd = new HashMap<>();
    for (int i = 1; i < s1.length(); i += 2) {
      map1Odd.merge(s1.charAt(i), 1L, Long::sum);
      map2Odd.merge(s2.charAt(i), 1L, Long::sum);
    }
    if (map1Even.size() != map2Even.size() || map1Odd.size() != map2Odd.size()) {
      return false;
    }
    return map1Even.equals(map2Even) && map1Odd.equals(map2Odd);
  }

  void main() {
    assert checkStrings("abcd", "cdab");
    assert checkStrings("abcdba", "cabdab");
    assert !checkStrings("abcd", "dacb");
    assert !checkStrings("abe", "bea");
  }
}
