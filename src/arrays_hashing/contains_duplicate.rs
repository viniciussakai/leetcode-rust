use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for num in nums {
        if hash.contains_key(&num) {
            return true;
        }
        hash.insert(num, 1);
    }

    return false;
}
