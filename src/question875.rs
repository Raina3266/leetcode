pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut piles = piles;
    let h_usize: usize = h.try_into().unwrap();
    piles.sort();
    let length = piles.len();
    if length == h_usize {
        return piles[length - 1];
    }

    let mut left = piles[0];
    let mut right = piles[length - 1];
    loop {
        if right - left <= 1 {
            if calculate_hours_for_one_speed(&piles, left) == h {
                return left;
            }
            return right
        }
        let midpoint = (right + left) / 2;

        if calculate_hours_for_one_speed(&piles, midpoint) <= h {
            right = midpoint;
        } else {
            left = midpoint;
        }
    }
}

fn calculate_hours_for_one_speed(piles: &[i32], speed: i32) -> i32 {
    let mut sum = 0;
    for p in piles {
        sum += p / speed + 1;
    }
    sum
}

#[test]
fn feature() {
    let piles = vec![3,6,7,11];
    assert_eq!(min_eating_speed(piles, 8), 4)
}