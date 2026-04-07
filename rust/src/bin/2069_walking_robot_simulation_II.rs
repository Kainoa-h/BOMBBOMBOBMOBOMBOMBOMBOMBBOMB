const DIR_VELS: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
const DIR_STRS: [&str; 4] = ["East", "North", "West", "South"];

struct Space {
    width_height: [i32; 2],
    perimeter: i32,
}

struct Robot {
    xy: [i32; 2],
    dir_idx: usize,
    space: Space,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Robot {
            xy: [0, 0],
            dir_idx: 0,
            space: Space {
                width_height: [width - 1, height - 1],
                perimeter: 2 * (width - 1) + 2 * (height - 1),
            },
        }
    }

    fn step(&mut self, num: i32) {
        let mut num = num % self.space.perimeter;
        if self.xy[0] == 0 && self.xy[1] == 0 && num == 0 {
            self.dir_idx = 3;
            return;
        }

        while num > 0 {
            let i = self.dir_idx & 1;
            let bounds = self.space.width_height[i];
            let vec = (DIR_VELS[self.dir_idx][i] * num) + self.xy[i];
            if vec < 0 {
                self.xy[i] = 0;
                self.dir_idx = (self.dir_idx + 1) % 4;
                num = i32::abs(vec);
            } else if vec > bounds {
                self.xy[i] = bounds;
                self.dir_idx = (self.dir_idx + 1) % 4;
                num = vec - bounds;
            } else {
                self.xy[i] = vec;
                break;
            }
        }
    }

    fn get_pos(&self) -> Vec<i32> {
        self.xy.to_vec()
    }

    fn get_dir(&self) -> String {
        DIR_STRS[self.dir_idx].to_string()
    }
}

fn main() {
    let mut r = Robot::new(6, 4);
    r.step(1);
    r.get_dir();
    r.get_pos();
}
