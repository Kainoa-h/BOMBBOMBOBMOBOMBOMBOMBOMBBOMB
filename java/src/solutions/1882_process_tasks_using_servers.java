import java.util.*;

class Solution {
  private static class Server implements Comparable<Server> {
    public final int id;
    public final int weight;
    public int time;

    public Server(int id, int weight) {
      this.id = id;
      this.weight = weight;
      this.time = 0;
    }

    @Override
    public int compareTo(Solution.Server o) {

      return Comparator.comparingInt((Server s) -> s.time)
          .thenComparingInt(s -> s.weight)
          .thenComparingInt(s -> s.id)
          .compare(this, o);
    }
  }

  public int[] assignTasks(int[] servers, int[] tasks) {
    int[] result = new int[tasks.length];
    PriorityQueue<Server> freeServersPQ = new PriorityQueue<>();
    PriorityQueue<Server> busyServersPQ = new PriorityQueue<>();
    for (int i = 0; i < servers.length; i++) {
      freeServersPQ.add(new Server(i, servers[i]));
    }

    int idx = 0;
    int seconds = 0;
    while (idx < tasks.length) {
      while (!busyServersPQ.isEmpty() && busyServersPQ.peek().time <= seconds) {
        Server s = busyServersPQ.poll();
        s.time = 0;
        freeServersPQ.add(s);
      }

      if (freeServersPQ.isEmpty()) {
        seconds = busyServersPQ.peek().time;
        continue;
      }

      Server s = freeServersPQ.poll();
      result[idx] = s.id;
      s.time = seconds + tasks[idx];
      busyServersPQ.add(s);
      idx++;
      seconds++;
    }

    return result;
  }
}
