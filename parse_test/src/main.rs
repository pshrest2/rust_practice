fn main() {
    let tuple: (u32, i64) = (18, -5 * 103948);

    println!("first: {}, second: {}", tuple.0, tuple.1);

    let array: [i32;5] = [1,2,3,4,5];
    println!("first: {}", array[array.len() - 1]);
}
