use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut no_of_guess = 0;
    let rand_no = rand::thread_rng().gen_range(1..=100);

    println!("Generated Number is = {} ", rand_no);

    loop {
        let mut guess = String::new();
        println!("Input The Guess : ");
        no_of_guess = no_of_guess + 1;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Warning !! Please Enter a Valid Number !");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&rand_no) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                println!("No of Guesses = {}", no_of_guess);
                break;
            }
        }
    }
}
