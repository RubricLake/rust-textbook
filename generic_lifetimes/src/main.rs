struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let a = [1, 2, 3];
    let mut it = a.into_iter();
    println!("Value: {:?}", it.next().unwrap());
}
