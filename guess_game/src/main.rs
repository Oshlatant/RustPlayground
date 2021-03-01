use random_number::random;
use std::{io, cmp::Ordering };



fn main() {
    let random_n: u8 = random!(..=100);
    let mut count: u8 = 0;
    loop {
        println!("\nGuess the number...");
        
        let mut guess = String::new();
        match io::stdin()
            .read_line(&mut guess) {
                Err(_) => continue,
                Ok(str) => str,
            };
            

        let guess: u8 = match guess.trim().parse() {
            Err(_) => continue,
            Ok(number) => number,
        };

        count += 1;

        match guess.cmp(&random_n) {
            Ordering::Greater => println!("Number is less !"),
            Ordering::Less => println!("Number is greater !"),
            Ordering::Equal => {
                println!("You won in {} try(s), secret number is {} !", count, random_n);
                break;
            },
        }
    }
    
}
