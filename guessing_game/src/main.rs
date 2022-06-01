use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Arvaa numero!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Anna numero: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Virhe numeron luvussa");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Liian pieni"),
            Ordering::Greater => println!("Liian suuri"),
            Ordering::Equal => {
                println!("Voitit!");
                break;
            }
        }

        //println!("Arvasit: {}", guess);

        //println!("Oikea arvaus olisi ollut: {}", secret_number);
    }
}
