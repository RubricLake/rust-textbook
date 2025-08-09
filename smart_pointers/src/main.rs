// Ch 15.2
struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(input: String) -> CustomSmartPointer {
        CustomSmartPointer { data: input }
    }
}

use std::ops::Deref;
impl Deref for CustomSmartPointer {
    type Target = String;
    
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn test_func(data: &String) { 
    println!("Given Data = {data}");
}

// Ch 15.3
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Ch 15.1 & 15.4 & 15.5
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};


#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), // Pointer for Indirection
    Nil,
}

fn main() {
    let x = CustomSmartPointer::new(String::from("Pointer #1"));
    let y = CustomSmartPointer::new(String::from("Pointer #2"));


    test_func(&x); // Deref Use

    drop(x);
    drop(y);


    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}