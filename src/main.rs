fn main() {
    let mut name = String::from("Pranaya");
    name.push_str(" Shrestha");

    println!("Full name is {name}");

    {
        let mut username = String::from("pshrest2");
        username.push_str("1234");
        println!("Username is {username}");
    }
    println!("Username outside the scope is {username}");
}