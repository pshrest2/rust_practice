fn main() {
    let name = String::from("Pranaya Shrestha");
    println!("Length of {name} is {}", length(&name));
}

fn length(variable: &String) -> usize {
    variable.len()
}
