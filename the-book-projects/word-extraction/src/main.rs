use std::io;

fn main() {
    println!("Word extractor");

    println!("Please enter a passage from which to extract:");
    let mut passage = String::new();

    io::stdin()
        .read_line(&mut passage)
        .expect("Failed to read line.");
    let passage = passage.trim();

    let first = first_word(passage);
    let second = second_word(passage);

    if second == "" {
        println!("You only entered one word, '{first}'.");
    } else {
        println!("The first word was '{first}', and the second word was '{second}'.");
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_over = false;
    let mut start = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if start > i && item == b' ' && !first_over {
            first_over = true;
        } else if start > i && first_over && item != b' ' {
            start = i;
        } else if item == b' ' {
            return &s[start..i];
        }
    }
    ""
}
