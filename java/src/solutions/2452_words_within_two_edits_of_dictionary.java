import java.util.ArrayList;
import java.util.List;

class Solution {
  public List<String> twoEditWords(String[] queries, String[] dictionary) {
    List<String> result = new ArrayList<>();
    for (var query : queries) {
      for (var word : dictionary) {
        if (matchWord(query, word)) {
          result.add(query);
          break;
        }
      }
    }
    return result;
  }

  boolean matchWord(String query, String word) {
    int errors = 0;
    for (int i = 0; i < query.length(); i++) {
      if (query.charAt(i) != word.charAt(i)) {
        errors++;
        if (errors > 2) {
          return false;
        }
      }
    }
    return true;
  }
}
