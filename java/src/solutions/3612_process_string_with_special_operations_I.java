class Solution {
  public String processStr(String s) {
    StringBuilder result = new StringBuilder();

    for (int c : s.chars().toArray()) {
      switch (c) {
        case int x when x == '*' && result.length() > 0 -> result.deleteCharAt(result.length() - 1);
        case '*' -> {}
        case '#' -> result.append(result.toString());
        case '%' -> result.reverse();
        default -> result.append((char) c);
      }
    }

    return result.toString();
  }
}
