use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let minus = target - num;

        print!("{}, ", minus);

        if hash.contains_key(&num) {
            return vec![*hash.get(&num).unwrap(), index as i32];
        }

        hash.insert(minus, index as i32);
    }

    vec![-1, -1]
}
