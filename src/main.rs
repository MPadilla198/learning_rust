fn main() {
    println!("Hello, world!");

    let strin = String::from("Hello, world!");

    let word = first_word(&strin);

    println!("{}", word);

    // strin.clear()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
