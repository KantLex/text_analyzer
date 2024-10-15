// std::io: For input/output operations.
// Write: To flush the output buffer.
use std::io::{self, Write};

fn main() {
    let text = read_input();

    //Passes references &text to avoid unnecessary copying.
    let char_count = count_characters(&text);
    let word_count = count_words(&text);
    let longest = longest_word(&text);

    println!("Character count: {}", char_count);
    println!("Word count: {}", word_count);
    match longest {
        Some(word) => println!("Longest word: '{}'", word),
        None => println!("No words found."),
    }
}

