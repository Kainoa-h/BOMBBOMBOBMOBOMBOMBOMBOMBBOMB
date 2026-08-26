class Solution {
    public String shortestBeautifulSubstring(String s, int k) {
        String best = null;
        var left = 0;
        var ones = 0;

        for (var right = 0; right < s.length(); right++) {
            ones += '1' == s.charAt(right) ? 1 : 0;
            if (ones == k) {
                while (s.charAt(left) == '0') {
                    left += 1;
                }
                var candidate = s.substring(left, right + 1);
                if (best == null
                        || candidate.length() < best.length()
                        || (candidate.length() == best.length() && candidate.compareTo(best) < 0)) {
                    best = candidate;
                }
                left += 1;
                ones -= 1;
            }
        }
        return best == null ? "" : best;
    }
}
