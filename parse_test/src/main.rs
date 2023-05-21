use std::io;

fn main() {
    println!("Hello, enter a number");

    let mut test = String::new();
    io::stdin().read_line(&mut test).expect("Could not read input from user");

    println!("You entered: {test}");
    println!("Converting string to integer...");

    let test: u8 = test.trim().parse().expect("Please enter a number");
    println!("The converted integer is: {test}");
}
