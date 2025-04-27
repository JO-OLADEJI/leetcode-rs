mod solutions;

use solutions::leetcode217;
use solutions::leetcode268;
use solutions::leetcode448;
use solutions::leetcode1;

fn main() {
    let mood = String::from("Leetcode Solutions :/");

    let nums: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;
    let answer = leetcode1::solution(&nums, &target);

    println!("{:?}", answer);
}
