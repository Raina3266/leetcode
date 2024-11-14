mod linked_list;

mod question14;
mod question20;
mod question26;
mod question27;
mod question28;
mod question35;
mod questin49;
mod question231;
mod question66;
mod question206;
mod question125;
mod question876;
mod question2260;
mod question2270;
mod question2342;
mod question2351;
mod question2352;
mod fibonacci;
mod question9;
mod question1;
mod two_lists;
pub mod is_substring;
mod reverse_string;
mod squares_of_sorted_array;
mod biggest_average;
mod subarray_averages;
mod max_consecutive_ones;
mod step_sum;
mod sum_of_1d_array;
mod subarray_product;
mod k_distinct_char;
mod missing_number;
mod count_elements;
mod length_of_longest_substring;
mod question1047;
mod question383;
mod question49;
mod question24;
mod question71;
mod question83;
mod question239;
mod question346;
mod question496;
mod question739;
mod question933;
mod question1189;
mod question525;
mod question560;
mod question771;
mod question844;
mod question901;
mod question1133;
mod question1248;
mod question1438;
mod question1544;
mod question1941;
mod question2225;
mod question2248;
fn main() {
    // let nums = [0,0,1,1,1,2,2,3,3,4]
    // let result = question26::remove_duplicates();
    // println!("{result}");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
  }
  
  impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
      ListNode {
        next: None,
        val
      }
    }
  }