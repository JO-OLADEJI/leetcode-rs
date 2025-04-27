use std::collections::HashMap;

// LEETCODE 1
// let x: Vec<i32> = vec![2,7,11,15]; target = 9  - [0,1]
// let y: Vec<i32> = vec![3,2,4];     target = 6  - [1,2]
// let z: Vec<i32> = vec![3,3];       target = 6  - [0,1]

pub fn solution(nums: &Vec<i32>, target: &i32) -> Vec<i32> {
    let mut diff: i32;
    let mut library: HashMap<i32, usize> = HashMap::new();

    for (i, value) in nums.iter().enumerate() {
        diff = target - value;

        match library.get(&diff) {
            Some(j) => {
                return vec![*j as i32, i as i32];
            }
            None => {
                library.insert(*value, i);
            }
        }
    }

    vec![]
}
