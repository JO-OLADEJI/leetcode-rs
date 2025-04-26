use std::collections::HashMap;

// LEETCODE 217
// let x: Vec<i32> = vec![1, 2, 3, 1];                   - true
// let y: Vec<i32> = vec![1, 2, 3, 4];                   - false
// let z: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]; - true

pub fn solution(nums: &Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();

    for num in nums {
        match map.get(&num) {
            Some(value) => {
                return true;
            }
            None => {
                map.insert(*num, true);
            }
        }
    }

    false
}
