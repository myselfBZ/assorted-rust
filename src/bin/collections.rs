fn main() {
    let words = vec![
        String::from("Bicycle"),
        String::from("Dolphin"),
        String::from("Falcon"),
        String::from("Guitar"),
        String::from("Hammer"),
        String::from("Jungle"),
        String::from("Kangaroo"),
        String::from("Lantern"),
        String::from("Mountain"),
        String::from("Notebook"),
        String::from("Pencil"),
        String::from("Question"),
        String::from("Rhythm"),
        String::from("Symphony"),
        String::from("Telescope"),
        String::from("Apple"),
        String::from("Eagle"),
        String::from("Island"),
        String::from("Ocean"),
        String::from("Umbrella"),
    ];
    let mut new_words : Vec<String> = Vec::new();

    for w in words.iter() {
        if !starts_with_consonant(w) {
            let mut new_str = String::from(w);
            new_str.push_str("-hay");
            new_words.push(new_str);
        } else {
            let mut new_str = String::from(&w[1..]);
            if let Some(first_char) = w.chars().next() {
                new_str.push_str(&format!("-{first_char}ay"));
                new_words.push(new_str);
            }
        }
    }


    for w in new_words {
        println!("{w}")
    }
}


fn starts_with_consonant(s: &str) -> bool {
    if let Some(first_char) = s.chars().next() {
        if first_char.is_alphabetic() {
            let lower = first_char.to_ascii_lowercase();
            return !matches!(lower, 'a' | 'e' | 'i' | 'o' | 'u');
        }
    }
    false
}
