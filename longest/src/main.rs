/*
 * Rust book - Chapter 10 Lifecycle
 * https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
 */


fn longest(string_a : &str, string_b : &str) -> &str {
    return string_a;
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
