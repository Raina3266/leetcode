
pub fn sum_of_1d_array(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = vec![];
    let mut item = 0;
    for n in 0..nums.len() {
        item += nums[n];
        sum.push(item.try_into().unwrap());
    }
    sum
}