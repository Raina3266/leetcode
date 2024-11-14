
// pub fn is_valid(s: String) -> bool {
//     let mut brackets = Vec::new();
//     for x in s.chars() {
//         match x {
//             '{' => brackets.push(Kind::Squiggly),
//             '[' => brackets.push(Kind::Square),
//             '(' => brackets.push(Kind::Round),
//             ')' => {
//                 let last_thing = brackets.pop();
//                 if last_thing != Some(Kind::Round) {
//                     return false
//                 }
//             },
//             ']' => {
//                 let last_thing = brackets.pop();
//                 if last_thing != Some(Kind::Square) {
//                     return false
//                 }
//             },
//             '}' => {
//                 let last_thing = brackets.pop();
//                 if last_thing != Some(Kind::Squiggly) {
//                     return false
//                 }
//             },
//             _ => unreachable!(),
//         }
//     }
//     brackets.is_empty()
// }

// #[derive(PartialEq)]
// enum Kind {
//     Round, 
//     Squiggly, 
//     Square,
// }


pub fn is_valid_2(s: String) -> bool {
    let collection: Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = vec![];
    
    for c in collection{
        match c {
            '[' | '{' | '(' => stack.push(c),
            ')' => {
                match stack.pop() {
                    Some(x) => {
                        if x != '('{
                            return false;
                        }
                    },
                    None => return false,
                }
            },
            '}' => {
                match stack.pop() {
                    Some(x) => {
                        if x != '{'{
                            return false;
                        }
                    },
                    None => return false,
                }
            },
            ']' => {
                match stack.pop() {
                    Some(x) => {
                        if x != '['{
                            return false;
                        }
                    },
                    None => return false,
                }
            },
            _ => unreachable!()
        }
    }
    stack.is_empty()
}
