fn main() {
    let variable = String::from("Pizza");
    let x = 7;

    takes_ownership(variable); // transfer of ownership. variable cannot be used after this line
    makes_copy(x); // x is cloned into the function. x can be used after this line

    println!("{variable}"); // this will result in compile error
    println!("{x}");
}   // x goes out of scope. drop is called on x to free the memory
    // drop is not called since it's already out of scope

fn takes_ownership(variable: String){ // variable comes into scope
    println!("{variable}");
} // variable goes out of scope. drop called on variable to free the memory

fn makes_copy(x: i32){ // x comes into scope
    println!("{x}");
} // x goes out of scope. drop is not called