use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    // New Empty Vector
    let mut v1: Vec<i32> = Vec::new();

    // Using vec! macro
    let v = vec![1, 2, 3 ,4, 5];

    // Adding to Vector
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // Indexing
    let _third: &i32 = &v[2];
    // println!("The third element is {_third}! (Bracket Indexing)");

    let third: Option<&i32> = v.get(20);
    if let Some(_value) = third {
        // println!("The third element is {_value:?}! (v.get)");
    }
    else {
        // println!("Out of bounds!");
    }

    // Mutability Concerns
    let mut v = vec![1,2, 3, 4, 5];
    let first = &mut v[0];
    *first = 5;    
    // v.push(6); <-- Error, mutable borrow while 'first' has immutability
    // println!("The first element is {first}!");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    
    // Way to store different data types in a vector
    let _row = vec![
        SpreadsheetCell::Int(69),
        SpreadsheetCell::Float(420.69),
        SpreadsheetCell::Text(String::from("Hell Yeah")),
    ];



    // Print Spreadsheet Row
    // print_row(&_row);

    /* String Section 8.2 */

    // These are equivalent
    let _s = String::from("initial content");
    let _s = "intial content".to_string();

    // Strings are UTF-8 Encoded so this is valid
    let _hello = String::from("こんにちは");

    // Updating a String
    let mut _s = String::from("foo");
    _s.push_str("bar");

    let mut _s1 = String::from("foo");
    let _s2 = "bar";
    _s1.push_str(_s2); // Does not take ownership
    // println!("_s2 is {_s2}");

    let mut _s = String::from("lo");
    _s.push('l');

    // Concatenation Works...
    let _s1 = String::from("tic");
    let _s2 = String::from("tac");
    let _s3 = String::from("toe");

    // let _s = _s1 + "-" + &_s2 + "-" + &_s3;

    // But format! is better!
    let _s = format!("{_s1}-{_s2}-{_s3}");


    // Since Strings aren't always consistent in byte size. We do this to iterate over them
    for _c in "Зд".chars() {
        // println!("{_c}");
}

    // If you need bytes, you can do this
    for _c in "Зд".bytes() {
    // println!("{_c}");
}

    /* Hash Map Section 8.3 */
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get Value from HashMap
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);
    
    // Iterate with For
    for (_key, _value) in &scores {
        // println!("{_key}: {_value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("Field Name: {field_name}"); ERROR!
    // println!("Field Value: {field_value}"); ERROR!

    // Updating a hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Overrides Previous Value

    // println!("{scores:?}");

    // Adding only if key isn't present
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    // println!("{scores:?}");

    // Update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // println!("Map: {map:?}");

    // Exercises 
    let odd_list = vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24]; // (15, 13)
    let (o_median, o_mode) = median_mode(&odd_list);
    println!("Median (Odd): {o_median}\nMode (Odd): {o_mode}");

    let my_word = String::from("Rust");
    println!("{my_word} in pig latin is {}!", to_pig_latin(&my_word));

    // Department thing
    clear();
    println!("Welcome to the departmant management service!");

    let mut input = String::new();
    let mut db: HashMap<String, String> = HashMap::new();

    loop {
        print_main_menu();
        
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid Input");

        match input.trim() {
            "1" => add_user(&mut db),
            "2" => print_users(&mut db),
            "3" => break, // Quit
            _ => { println!("Illegal Command. Try again.");
                    continue;
                }
        }
    }
    

    }

    fn print_users(db: &mut HashMap<String, String>) {
        clear();
        println!("{db:?}");
    }

    fn add_user(db: &mut HashMap<String, String>) {
        let mut name = String::new();
        let mut department = String::new();
        

        print!("Name of Individual: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut name)
            .expect("Invalid Input (Name)");

        print!("Name of Department: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut department)
            .expect("Invalid Input (Department)");
        
        let name = name.trim().to_string();
        let department = department.trim().to_string();

        db.insert(name.clone(), department.clone());
        clear();
        println!("{name} added to {department} successfully!");
    }

    fn print_main_menu() {
        println!("Please select an option!\n
                    1). Add person to department\n
                    2). View all records\n
                    3). Quit\n");
    }

    fn median_mode(list: &Vec<i32>) -> (i32, i32) {
        let mut map = HashMap::new();

        for num in list {
            let count = map.entry(*num).or_insert(0);
            *count += 1;
        }

        let mut max_key = 0;
        let mut max_val = 0;

        for (key, val) in map {
            if val > max_val {
                max_key = key;
                max_val = val;
            }
        }

        let list_copy = list.clone();
        let len = list_copy.len();
        
        
        (list_copy[len / 2], max_key)
    }
    
    fn to_pig_latin(word: &String) -> String {
        let first_char = word.chars().nth(0).unwrap();

        match first_char.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", &word[..]),
            _ => format!("{}-{}ay", &word[1..], first_char),
        }
    }

    fn clear() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

#[allow(dead_code)]
fn print_row(row: &Vec<SpreadsheetCell>) {
    for field in row {
        match field {
            SpreadsheetCell::Int(value) => println!("int Value = {value}"),
            SpreadsheetCell::Float(value) => println!("Float Value = {value}"),
            SpreadsheetCell::Text(value) => println!("Text Value = {}", value),
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


