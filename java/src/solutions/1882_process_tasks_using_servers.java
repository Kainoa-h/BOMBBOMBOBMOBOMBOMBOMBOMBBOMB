import java.util.*;

class Solution {
  private static class Server {
    public final int id;
    public final int weight;
    public int time;

    public Server(int id, int weight) {
      this.id = id;
      this.weight = weight;
      this.time = 0;
    }
  }

  public int[] assignTasks(int[] servers, int[] tasks) {
    int[] result = new int[tasks.length];
    PriorityQueue<Server> freeServersPQ = new PriorityQueue<>((a, b) -> {
      if (a.weight != b.weight) {
        return a.weight - b.weight;
      }
      return a.id - b.id;
    });
    PriorityQueue<Server> busyServersPQ = new PriorityQueue<>((a, b) -> a.time - b.time);
    for (int i = 0; i < servers.length; i++) {
      freeServersPQ.add(new Server(i, servers[i]));
    }

    int idx = 0;
    int seconds = 0;
    while (idx < tasks.length) {
      while (!busyServersPQ.isEmpty() && busyServersPQ.peek().time <= seconds) {
        freeServersPQ.add(busyServersPQ.poll());
      }

      if (freeServersPQ.isEmpty()) {
        seconds = busyServersPQ.peek().time;
        continue;
      }

      while (idx <= seconds && !freeServersPQ.isEmpty() && idx < tasks.length) {
        Server s = freeServersPQ.poll();
        result[idx] = s.id;
        s.time = seconds + tasks[idx];
        busyServersPQ.add(s);
        idx++;
      }

      seconds++;
    }

    return result;
  }
}
