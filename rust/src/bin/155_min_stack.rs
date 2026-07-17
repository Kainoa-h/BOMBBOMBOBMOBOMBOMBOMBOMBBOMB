struct MinStack {
    stack: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
        if let Some(&min) = self.min.last() {
            if value <= min {
                self.min.push(value);
            }
        } else {
            self.min.push(value);
        }
    }

    fn pop(&mut self) {
        if let Some(v) = self.stack.pop() && let Some(&m) = self.min.last() && m == v {
            self.min.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}

fn main(){}
