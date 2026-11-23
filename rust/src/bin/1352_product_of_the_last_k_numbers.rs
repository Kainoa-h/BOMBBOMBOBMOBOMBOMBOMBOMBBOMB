struct ProductOfNumbers {
    numbers: Vec<u32>
}

impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            numbers: Vec::new(),
        }
    }
    
    fn add(&mut self, num: i32) {
        self.numbers.push(num as u32);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let n = self.numbers.len() - k as usize;
        self.numbers[n..].iter().product::<u32>() as i32
    }
}
