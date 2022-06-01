fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("Another function. x is {}", x);
}

fn main() {
    println!("Hello, world!");
    another_function(five());
    print_labeled_measurement(5,'h');
}



