class Solution {
  public long minimumCost(int cost1, int cost2, int costBoth, int need1, int need2) {
    final long cost12 = cost1 + cost2;
    int sharedNeeds = Math.min(need1, need2);
    need1 -= sharedNeeds;
    need2 -= sharedNeeds;

    long sharedCost = sharedNeeds * (long) Math.min(costBoth, cost12);
    long need1Cost = need1 * (long) Math.min(costBoth, cost1);
    long need2Cost = need2 * (long) Math.min(costBoth, cost2);
    return sharedCost + need1Cost + need2Cost;
  }
}
