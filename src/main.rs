// std::io: For input/output operations.
// Write: To flush the output buffer.
use std::io::{self, Write};

fn main() {
    let text: String = read_input();

    //Passes references &text to avoid unnecessary copying.
    let char_count: usize = count_characters(&text);
    let word_count: usize = count_words(&text);
    let longest: Option<&str> = longest_word(&text);

    println!("Character count: {}", char_count);
    println!("Word count: {}", word_count);
    match longest {
        Some(word) => println!("Longest word: '{}'", word),
        None => println!("No words found."),
    }
}

fn read_input() -> String {
    let mut text: String = String::new();

    println!("Enter your text (type 'END' to finsinh)");

    loop {
        print!(">");
        io::stdout().flush().expect("Failed to flush stdout");

        // Accumulates the text in a String.
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim() == "END" {
            break;
        }

        text.push_str(&line);
    }

    text
}

fn count_characters(text: &String) -> usize {
    text.chars().count()
}

fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}

fn longest_word<'a>(text: &'a String) -> Option<&'a str> {
    let mut longest: Option<&str> = None;

    for word in text.split_whitespace() {
        match longest {
            Some(current_longest) => {
                if word.len() > current_longest.len() {
                    longest = Some(word);
                }
            }
            None => {
                longest = Some(word);
            }
        }
    }

    longest
}