struct MinStack {
    stack: Vec<(i32,i32)>
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        let min = match self.stack.last() {
            Some(m) => m.1.min(value),
            None => value
        };
        self.stack.push((value, min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}

fn main(){}
