import java.util.HashMap;
import java.util.Map;
import java.util.List;
import java.util.ArrayList;

class Solution {
  public long[] distance(int[] nums) {
    long[] result = new long[nums.length];
    Map<Integer, List<Long>> numMap = new HashMap<>();

    for (int i = 0; i < nums.length; i++) {
      numMap.computeIfAbsent(nums[i], k -> new ArrayList<Long>()).add(Long.valueOf(i));
    }

    for (int i = 0; i < nums.length; i++) {
      var idx = i;
      result[i] = numMap.get(nums[i]).stream().mapToLong(x -> Math.abs(idx - x)).sum();
    }
    return result;
  }
}
