/*
 * The sequence of numbers, starting with zero and one,
 * is created by adding the previous two numbers. For example,
 * the early part of the sequence is
 * 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89,144, 233, 377, and so on.
 */

// Return n:t fibonacci number
fn fibonacci(mut n: u32) -> u32 {

    let mut old_number = 0;
    let mut new_number = 1;
    let mut temp;

    while n > 0 {
        temp = new_number;
        new_number += old_number;
        old_number = temp;
        println!("number =  {}", new_number);
        n -= 1;
    }

    return new_number;
}

fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("fibonacci member {} is {}", n, result);
}
