fn main() {
    let user = build_user(String::from("agraham"), String::from("agraham@stjude.org"));
    let is_or_not = if user.active { String::from("is") } else { String::from("is not") };
    println!("{} {is_or_not} active and has logged in {} times with email {}", user.username, user.sign_in_count, user.email);
}

fn build_user (username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}