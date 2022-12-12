use std::io;

fn passage_to_words(passage_slice: &str) -> Vec<String> {
    let passage = String::from(passage_slice);
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    for i in passage.chars() {
        if i == ' ' {
            words.push(word);
            word = String::new();
        } else {
            word.push(i)
        }
    }
    words.push(word);
    words
}

fn pigify(word: String) -> String {
    let vowels = ['A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u'];
    let punct = ['.', ',', ';', ':', '\'', '"'];
    let mut chars: Vec<char> = word.chars().collect();
    let mut punctuation: Vec<char> = Vec::new();
    loop {
        if punct.contains(&chars[chars.len() - 1]) {
            punctuation.insert(0, chars.pop().unwrap());
        } else {
            break;
        }
    }
    if vowels.contains(&chars[0]) {
        chars.push('h');
    } else {
        let first = chars.remove(0);
        chars.push(first);
    }
    chars.push('a');
    chars.push('y');
    for p in punctuation {
        chars.push(p);
    }
    chars.into_iter().collect()
}

pub fn main() {
    println!("Pig Latin");
    println!("Please enter a passage to convert.");
    let mut passage = String::new();

    io::stdin()
        .read_line(&mut passage)
        .expect("Failed to read line.");
    let words = passage_to_words(passage.trim());

    let mut pig_passage: Vec<String> = Vec::new();
    for word in words {
        pig_passage.push(pigify(word));
    }
    println!("The passage in pig latin is:");
    println!("{}", pig_passage.join(" "));
}