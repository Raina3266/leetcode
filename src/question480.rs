
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let mut new_nums = nums;
    let mut ans: Vec<f64> = vec![];
    let usize_k: usize = k.try_into().unwrap();
    for i in 0..=new_nums.len() - usize_k {
        let window = &new_nums[i..i+usize_k];
        println!("{window:?}");
        let median = find_median_for_one_window(window);
        ans.push(median);
    }
    ans
}


fn find_median_for_one_window(input: &[i32]) ->f64 {
    let length = input.len();
    let mut new_input = input.to_vec();
    new_input.sort();
    if length % 2 == 1 {
        
        let f = new_input[length / 2] as f64;
        (f * 100000.0).round() / 100000.0
    } else {
        let right = new_input[length / 2] as f64;
        let left = new_input[length / 2 - 1] as f64;
        
        let f = (left + right) / 2.0;
        (f * 100000.0).round() / 100000.0
    }
}

#[test]
fn feature() {
    let nums = vec![1,3,-1,-3,5,3,6,7];
    assert_eq!(median_sliding_window(nums, 3), vec![1.000, 2.000]);

}