use std::{cmp::{max, Reverse}, collections::BinaryHeap};

pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> { 
    let usize_k: usize = k.try_into().unwrap();
    let window = &nums[0..usize_k];
    let mut max_heap: BinaryHeap<i32> = BinaryHeap::from(window.to_vec());
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    balance_two_heaps(&mut max_heap, &mut min_heap);

    for i in 0..=nums.len() - usize_k {
        max_heap.push(nums[i + usize_k - 1]);
        balance_two_heaps(&mut max_heap, &mut min_heap);
    }



    todo!()
}

fn balance_two_heaps(max_heap: &mut BinaryHeap<i32>, min_heap: &mut BinaryHeap<Reverse<i32>>) {
    loop {
        min_heap.push(Reverse(max_heap.pop().unwrap()));
        if min_heap.len() > max_heap.len() {
            max_heap.push(min_heap.pop().unwrap().0);
            return
        }
    }
}

// pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
//     let mut new_nums = nums;
//     let mut ans: Vec<f64> = vec![];
//     let usize_k: usize = k.try_into().unwrap();
//     for i in 0..=new_nums.len() - usize_k {
//         let window = &new_nums[i..i+usize_k];
//         let median = find_median_for_one_window(window);
//         ans.push(median);
//     }
//     ans
// }


// fn find_median_for_one_window(input: &[i32]) ->f64 {
//     let length = input.len();
//     let mut new_input = input.to_vec();
//     new_input.sort();
//     if length % 2 == 1 {
        
//         let f = new_input[length / 2] as f64;
//         (f * 100000.0).round() / 100000.0
//     } else {
//         let right = new_input[length / 2] as f64;
//         let left = new_input[length / 2 - 1] as f64;
        
//         let f = (left + right) / 2.0;
//         (f * 100000.0).round() / 100000.0
//     }
// }
