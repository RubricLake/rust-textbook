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

// Ch 15.1 & 15.4
use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List<T> {
    Cons(T, Rc<List<T>>), // Use Pointer for Indirection
    Nil,
}

fn main() {
    let x = CustomSmartPointer::new(String::from("Pointer #1"));
    let y = CustomSmartPointer::new(String::from("Pointer #2"));


    test_func(&x); // Deref Use

    drop(x);
    drop(y);


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}