// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] 
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

use std::collections::HashMap;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    for num in nums {
        map.insert(*num, ind);
    }
    todo!()
}

pub fn two_sum_i(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        let mut new_nums = vec![];
        if map.contains_key(n) {
            let index = map.get(n).unwrap();
            ans.push((n, *index));
        }
        let needed_num = target - n;
        map.insert(needed_num, i);
    }
    ans
}
