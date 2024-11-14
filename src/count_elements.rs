use std::collections::HashSet;

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let sett: HashSet<i32> = HashSet::from_iter(arr.clone().into_iter());
    let mut count = 0;

    for n in arr {
        if sett.contains(&(n+1)) {
            count += 1;
        }
    }
    count
}