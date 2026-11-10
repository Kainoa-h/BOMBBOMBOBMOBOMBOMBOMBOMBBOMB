import java.util.Arrays;
import java.util.function.BiFunction;

class Solution {
    public int climbStairs(int n, int[] costs) {
        BiFunction<Integer, Integer, Integer> calc_cost = (from, to) -> {
            Double x = costs[to - 1] + Math.pow((to - from), 2);
            return x.intValue();
        };
        int[] dp = new int[n + 1];
        Arrays.fill(dp, 1, n + 1, Integer.MAX_VALUE);
        for (int step = 0; step < dp.length - 1; step++) {
            var currCost = dp[step];
            for (int nextStep = step + 1; nextStep <= step + 3 && nextStep < dp.length; nextStep++) {
                var stepCost = calc_cost.apply(step, nextStep);
                dp[nextStep] = Math.min(dp[nextStep], currCost + stepCost);
            }
        }

        return dp[n];
    }
}
