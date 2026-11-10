class Solution {
    public int climbStairs(int n, int[] costs) {
        if (n == 1) return costs[0] + 1;
        var thirdStepBack = 0;
        var secondStepBack = costs[0] + 1;
        var firstStepBack = costs[1] + Math.min(secondStepBack + 1, 4);
        var result = firstStepBack;
        for (int step = 3; step <= n; step++) {
            var oneStep = firstStepBack + 1;
            var twoStep = secondStepBack + 4;
            var threeStep = thirdStepBack + 9;
            thirdStepBack = secondStepBack;
            secondStepBack = firstStepBack;
            firstStepBack = result = costs[step - 1] + Math.min(oneStep, Math.min(twoStep, threeStep));
        }

        return result;
    }
}
