fn main() {
    let sentence = String::from("  hello world");
    let first_word = first_word(&sentence);
    println!("First word is {first_word}");
}

fn first_word(sentence: &String) -> &str {
    let trimmed = sentence.trim();
    let bytes = trimmed.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &trimmed[0..i];
        }
    }

    &trimmed[..]
}