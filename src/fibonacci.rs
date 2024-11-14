// 0, 1, 1, 2, 3, 5, 8, 13, 21, ...

// to get the next number, you add the previous 2

// write a function ⁠ fn fib(n: i32) -> i32 ⁠ that calculates the nth fibonacci number:

//  - fib(0) = 0
//  - fib(1) = 1
//  - fib(2) = 1
//  - ...
//  - fib(8) = 21

pub fn fib(n: i32) -> i32 {
    if n == 0 { 
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub fn fib2(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib2(n - 1) + fib2(n - 2)
    }
}

#[test]
fn foo() {
    assert_eq!(fib(8), 21);
    assert_eq!(fib(11), 89);
}