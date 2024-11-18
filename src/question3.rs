use std::collections::HashSet;


pub fn length_of_longest_substring(s: String) -> i32 {
    let s_list: Vec<char> = s.chars().collect();
    let mut sett: HashSet<char> = HashSet::new();

    let mut left = 0;
    let mut right = 0;
    let mut answer = 0;
    while left < s_list.len() && right < s_list.len() {
       if sett.insert(s_list[right]) {
            right += 1;
       } else {
            sett.remove(&s_list[left]);
            left += 1
       }
       
       answer = std::cmp::max(answer, (right - left) as i32)
    }
    answer
}

// aaabbbbccc
//r  1
//l 1
// {}