fn main() {
    let filter = FilterCondition {
        condition: Box::new(|x| x % 2 == 0),
    };
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filtered_data = filter.custom_filter(&data);
    println!("{:?}", filtered_data);
}

struct FilterCondition {
    condition: Box<dyn Fn(u8) -> bool>,
}

impl FilterCondition {
    fn is_match(&self, el: &u8) -> bool {
        (self.condition)(*el)
    }

    fn custom_filter(&self, vec: &Vec<u8>) -> Vec<u8> {
        vec.iter().filter(|x| self.is_match(x)).cloned().collect()
    }
}
