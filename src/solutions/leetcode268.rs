// LEETCODE 268
// let x: Vec<i32> = vec![3,0,1];             - 2
// let y: Vec<i32> = vec![0,1];               - 2
// let z: Vec<i32> = vec![9,6,4,2,3,5,7,0,1]; - 8

pub fn solution(nums: &Vec<i32>) -> i32 {
    let mut remainder: i32 = ((nums.len() * (nums.len() + 1)) / 2) as i32;

    for num in nums {
        remainder = remainder - num;
    }

    remainder
}
