import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Solution {
  public int[] dailyTemperatures(int[] temperatures) {
    List<Integer> monotonicStack = new ArrayList<>(temperatures.length);
    int[] result = new int[temperatures.length];

    int curr_idx = 0;
    for (var curr_temp : temperatures) {
      while (!monotonicStack.isEmpty() && temperatures[monotonicStack.getLast()] < curr_temp) {
        var top_idx = monotonicStack.removeLast();
        result[top_idx] = curr_idx - top_idx;
      }
      monotonicStack.add(curr_idx);
      curr_idx++;
    }

    return result;
  }

  void main() {
    System.out.println(Arrays.toString(new Solution().dailyTemperatures(new int[] { 30, 40, 50, 60 })));
    System.out.println(Arrays.toString(new int[] { 1, 1, 1, 0 }));
  }
}

/*
 * Example 1:
 * 
 * Input: temperatures = [73,74,75,71,69,72,76,73]
 * Output: [1,1,4,2,1,1,0,0]
 * Example 2:
 * 
 * Input: temperatures = [30,40,50,60]
 * Output: [1,1,1,0]
 * Example 3:
 * 
 * Input: temperatures = [30,60,90]
 * Output: [1,1,0]
 * 
 */
