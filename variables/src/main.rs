/* Chapter 3 Homework */

// Problem 1
fn celsius_to_fahrenheit(degree_c : f64) -> f64 { (degree_c * 1.8) + 32.0 }

fn fahrenheit_to_celsius(degree_f : f64) -> f64 { (degree_f - 32.0) / 1.8 }

// Problem 2
fn nth_fibonacci(mut n : u32) -> u32 {
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }

    let mut previous = 0;
    let mut current = 1;

    for _  in 2..=n {
        let next = previous + current;
        previous = current;
        current = next;
    }

    current
}

// TODO: Problem 3

fn main() {
  

    let degree_c = 22.0;
    let degree_f = 91.0;
    let n : u32 = 7;

    println!("{} C = {:.2} F", degree_c, celsius_to_fahrenheit(degree_c));
    println!("{} F = {:.2} C", degree_f, fahrenheit_to_celsius(degree_f));
    println!("Fibonacci #{} = {}", n, nth_fibonacci(n));
}
