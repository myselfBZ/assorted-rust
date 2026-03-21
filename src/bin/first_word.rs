



fn main() {
    let s = String::from("Hello, World");
    let first_word_s = first_word(&s);

    println!("first word is: {}", first_word_s);
}

fn first_word(s: &str) -> &str {
    for (index, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..index]
        }
    }

    return &s[..]
}
