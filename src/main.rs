fn main() {
    let mut index = 0;
    let numbers = ["spider","man", "=", "peter","parker"];

    while index < numbers.len() {
        print!("{} ", numbers[index]);
        index += 1;
    }
}