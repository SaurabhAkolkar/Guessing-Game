use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
   
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Something Goes Wrong..");

        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Less"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
