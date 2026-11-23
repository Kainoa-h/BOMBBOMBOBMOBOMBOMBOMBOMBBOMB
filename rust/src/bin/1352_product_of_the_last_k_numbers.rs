struct ProductOfNumbers {
    prefix_product: Vec<i32>
}

impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            prefix_product: vec![1],
        }
    }
    
    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_product.clear();
            self.prefix_product.push(1);
            return;
        }
        let prod = self.prefix_product.last().unwrap() * num;
        self.prefix_product.push(prod);
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k >= self.prefix_product.len() {
            return 0;
        }
        let last = self.prefix_product.last().unwrap();
        let ex_idx = self.prefix_product.len() - k - 1 ;
        let exclude = self.prefix_product[ex_idx];
        last / exclude
    }
}
