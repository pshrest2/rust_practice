use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your number: ");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Something went wrong");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Great Job!");
                break;
            }
        }
    }
}
