fn main() {
    let mut x = 5;
    let mut y = x;
    x += 1;
    y += 2;

    println!("Value of x is {x} ");
    println!("Value of y is {y} ");


    let mut user1 = String::from("Peter");
    let mut user2 = user1;

    user1.push_str(" Parker");
    
    println!("User 1 is {user1}");
    println!("User 2 is {user2}");
}