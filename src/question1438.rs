pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;
    let mut maximum: Vec<i32> = vec![];
    let mut minimum: Vec<i32> = vec![];
    while left < nums.len() && right < nums.len(){
        if maximum.is_empty() {
            maximum.push(nums[right])
        }
        if minimum.is_empty() {
            minimum.push(nums[right])
        }
        right += 1;
        if nums[right] > *maximum.last().unwrap() {
            
        }
    }
    ans
}