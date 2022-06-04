fn main() {
    // new static string
    let static_string = String::from("static string");

    // new mutable string
    let mut mutable_string = String::from("mutable string");

    // after this line static string is moved to other string and static string is not available anymore
    //let other_string = static_string;

    // using & creates reference and static_string is still available after "borrow"
    let other_string =  &static_string;


    // referencing mutable string
    let mut other_mutable_string = &mut mutable_string;
    // this works and returns the ownership back to mutable_string
    println!("Other mutable string: {}", other_mutable_string);

    println!("{}", static_string);
    println!("{}", mutable_string);
    println!("Other string: {}", other_string);
    
    // This does not work since mutable can be active only at one place at the time
    //println!("Other mutable string: {}", other_mutable_string);

}
