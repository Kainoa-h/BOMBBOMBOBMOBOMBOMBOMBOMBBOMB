class Solution {
    public int largestInteger(int[] nums, int k) {
        if (k == 1) {
            var bucket = new int[51];
            for (int n : nums) {
                bucket[n]++;
            }
            int max = -1;
            for (int i = 0; i < bucket.length; i++) {
                if (bucket[i] == 1) {
                    max = Integer.max(max, i);
                }
            }
            return max;
        } else if (k == nums.length) {
            int max = -1;
            for (int n : nums) {
                max = Integer.max(max, n);
            }
            return max;
        } else {
            var first = nums[0];
            var last = nums[nums.length - 1];
            if (first == last) {
                return -1;
            }
            var is_first = true;
            var is_last = true;
            for (int i = 1; i < nums.length - 1; i++) {
                if (nums[i] == first) {
                    is_first = false;
                } else if (nums[i] == last) {
                    is_last = false;
                }
            }
            first = is_first ? first : -1;
            last = is_last ? last : -1;
            return Integer.max(first, last);
        }
    }
}
