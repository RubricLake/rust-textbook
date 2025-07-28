fn main() {
    let  my_string = String::from("hello world");

    let word = first_word(&my_string);
    let word2 = first_word("hello2 world");

    println!("Word = {word}");
    println!("Word2 = {word2}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}