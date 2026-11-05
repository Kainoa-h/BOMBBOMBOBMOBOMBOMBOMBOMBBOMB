import java.util.*;

record State(int encoding, int weight) implements Comparable<State> {
  @Override
  public int compareTo(State o) {
    return Integer.compare(this.weight, o.weight);
  }
}

class Solution {
  public int shortestPath(int n, int[][] edges, String labels, int k) {
    int[] edgesBucket = new int[n];
    for (var edge : edges) {
      edgesBucket[edge[0]]++;
    }

    int[] adjNodeStart = new int[n + 1];
    var prefixSum = 0;
    for (var i = 0; i < n; i++) {
      adjNodeStart[i] = prefixSum;
      prefixSum += edgesBucket[i];
    }
    adjNodeStart[n] = prefixSum;

    int[] adjTo = new int[edges.length];
    int[] adjWeights = new int[edges.length];

    for (var edge : edges) {
      var from = edge[0];
      var idx = adjNodeStart[from] + --edgesBucket[from];
      var to = edge[1];
      adjTo[idx] = to;
      var weight = edge[2];
      adjWeights[idx] = weight;
    }

    int[] distances = new int[n * k];
    Arrays.fill(distances, k, n * k, Integer.MAX_VALUE);
    PriorityQueue<State> pq = new PriorityQueue<>();
    pq.add(new State(0, 0));
    while (!pq.isEmpty()) {
      var state = pq.poll();
      if (state.weight() != distances[state.encoding()]) {
        continue;
      }
      var node = state.encoding() / k;
      var count = state.encoding() % k;
      if (node == n - 1) {
        return state.weight();
      }
      var currChar = labels.charAt(node);
      for (var eIdx = adjNodeStart[node]; eIdx < adjNodeStart[node + 1]; eIdx++) {
        var weight = adjWeights[eIdx];
        var sumWeight = weight + state.weight();
        var to = adjTo[eIdx];
        var newCount = 0;
        if (labels.charAt(to) == currChar) {
          newCount = count + 1;
        }
        if (newCount == k) {
          continue;
        }
        var encoding = to * k + newCount;
        if (distances[encoding] > sumWeight) {
          distances[encoding] = sumWeight;
          pq.add(new State(encoding, sumWeight));
        }
      }
    }

    return -1;
  }
}
