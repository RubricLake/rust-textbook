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

/* String Section 8.2 */