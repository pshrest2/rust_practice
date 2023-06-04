fn main() {
    let some_string = no_dangle();
    println!("Some string is {some_string}");
}

fn no_dangle() -> String {
    let s = String::from("some string");
    s
}