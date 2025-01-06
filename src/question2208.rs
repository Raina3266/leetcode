use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]

struct F64Ord(f64);

impl Eq for F64Ord {}

impl Ord for F64Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn halve_array(nums: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    let mut sum = 0.0;
    let mut nums_f64 = vec![];
        for i in 0..nums.len() {
            sum += nums[i] as f64;
            nums_f64.push(F64Ord(nums[i] as f64));
        }
    let mut heap: BinaryHeap<F64Ord> = BinaryHeap::from(nums_f64);
    let half = sum / 2.0;
    let mut ans = 0;

    while sum > half {
        let max = heap.pop().unwrap().0;
        heap.push(F64Ord(max/2.0));
        ans += 1;
        sum -= max/2.0;
    }
    ans
}