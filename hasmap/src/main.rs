fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(i) => println!("score: {}", i),
        None => (),
    }
    //println!("score: {}", score);

}
