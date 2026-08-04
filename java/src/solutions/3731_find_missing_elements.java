import java.util.ArrayList;
import java.util.List;

class Solution {
    public List<Integer> findMissingElements(int[] nums) {
        boolean[] bucket = new boolean[101];
        int min = 101;
        int max = 0;
        for (int n : nums) {
            bucket[n] = true;
            min = Math.min(min, n);
            max = Math.max(max, n);
        }
        List<Integer> result = new ArrayList<>();
        for (int i = min + 1; i < max; i++){
            if (!bucket[i]){
                result.add(i);
            }
        }
        return result;
    }
}
