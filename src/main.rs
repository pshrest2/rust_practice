fn main() {
    let sentence = String::from("  hello world");
    let first_word = first_word(&sentence);

    println!("First word is {first_word}");

    let array = [1,2,3,4,5];
    let sliced_array = &array[1..3];
    assert_eq!(sliced_array, &[2,3], "Testing that slices matches");
}

fn first_word(sentence: &str) -> &str {
    let trimmed = sentence.trim();
    let bytes = trimmed.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &trimmed[0..i];
        }
    }

    &trimmed[..]
}