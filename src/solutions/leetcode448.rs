// LEETCODE 448
// let x: Vec<i32> = vec![4,3,2,7,8,2,3,1] - 2
// let x: Vec<i32> = vec![1,1];            - 2

pub fn solution(nums: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let j = (i32::abs(nums[i]) - 1) as usize;
        if nums[j] > 0 {
            nums[j] *= -1;
        }
    }

    return nums.into_iter().enumerate().filter(|(_, n)| **n > 0).map(|(i, _)| (i + 1) as i32).collect::<Vec<i32>>();
}
