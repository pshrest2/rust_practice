use std::io;

fn main() {
    let mut user_input: String = String::new();

    println!("Hey, have you had dinner yet? (Y/N)");
    match io::stdin().read_line(&mut user_input){
        Ok(_) => {
            user_input = user_input.trim().to_string();
            
            if user_input.eq_ignore_ascii_case("Y"){
                println!("Good :)");
            } else if user_input.eq_ignore_ascii_case("N") {
                println!("Sorry to hear that :(");
            } else {
                println!("Invalid input. Please try again later.");
            }
        }
        Err(error) => println!("error: {error}")
    }
}
