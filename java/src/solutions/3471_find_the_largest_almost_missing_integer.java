class Solution {
    public int largestInteger(int[] nums, int k) {
        var len = nums.length;
        if (k == 1) {
            var bucket = new int[51];
            for (int n : nums) {
                bucket[n]++;
            }
            for (int i = 50; i > 0; i--) {
                if (bucket[i] == 1) {
                    return i;
                }
            }
            return -1;
        }
        if (k == len) {
            int max = -1;
            for (int n : nums) {
                max = Integer.max(max, n);
            }
            return max;
        }

        var first = nums[0];
        var last = nums[len - 1];
        if (first == last) {
            return -1;
        }
        var seen_first = false;
        var seen_last = false;
        for (int i = 1; i < len - 1; i++) {
            if (nums[i] == first) seen_first = true;
            else if (nums[i] == last) seen_last = true;
            if (seen_first && seen_last) return -1;
        }
        first = seen_first ? -1 : first;
        last = seen_last ? -1 : last;
        return Integer.max(first, last);
    }
}
