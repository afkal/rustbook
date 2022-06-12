/*
 * Returns median and mode for the list of integers
 * median - when sorted, the value in the middle position
 * mode - the value that occurs most often; a hash map will be helpful here
 */
use std::collections::HashMap;

fn main() {
    // empty vector
    // let v: Vec<i32> = Vec::new();
    // create vector from values
    let mut v = vec![1, 3, 13, 6, 4, 3, 2, 19];

    // sort vector
    v.sort();

    let lenght = v.len();

    println!("median: {}", v[lenght/2]);

    // echo vector
    for i in &v {
        print!("{} ", i);
    }
    println!();

    let mut mode = HashMap::new();

    // create hashmap with vector values and number of occurances
    for i in &v {
        //mode.insert(i, 1);
        let count = mode.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", mode);


}
