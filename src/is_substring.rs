pub fn is_subsequence(s: String, t: String) -> bool {
    let ss: Vec<char> = s.chars().collect();
    let tt: Vec<char> = t.chars().collect();

    let mut i = 0;
    let mut j = 0;
    
    while i < ss.len() && j < tt.len() {
        if ss[i] == tt[j] {
            i += 1;
        } 
        j += 1;
    }
    i == ss.len()
}

#[test]
fn example_1() {
    let s = "ace";
    let t = "abcdeace";
    assert_eq!(is_subsequence(s.to_string(), t.to_string()), true);
}