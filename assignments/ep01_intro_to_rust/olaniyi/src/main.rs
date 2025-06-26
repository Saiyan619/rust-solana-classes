fn main() {
    println!("*SIGH!");
}

use std::io;
use rand::Rng;

// bring in the input/output library
use std::io;
// bring in the random number generator
use rand::Rng;

fn main() {
    // ask the user to enter a number
    println!("Please enter a number:");

    // make a new string to store the input
    let mut user_input = String::new();
    // read the input from the user
    io::stdin().read_line(&mut user_input).expect("Failed to read input");

    // change the input from text to a number (f64 means decimal numbers)
    let user_number: f64 = user_input.trim().parse().expect("Please type a number!");

    // create a random number between 1000 and 5000
    let random_number = rand::thread_rng().gen_range(1000.0..=5000.0);

    // print the random number
    println!("My number is: {}", random_number);

    // now call the functions to do math
    multiply_numbers(user_number, random_number);
    add_numbers(user_number, random_number);
    subtract_numbers(user_number, random_number);
}

// function to multiply the numbers
fn multiply_numbers(a: f64, b: f64) {
    let result = a * b;
    println!("Multiplication: {}", result);
}

// function to add the numbers
fn add_numbers(a: f64, b: f64) {
    let result = a + b;
    println!("Addition: {}", result);
}

// function to subtract the numbers
fn subtract_numbers(a: f64, b: f64) {
    let result = a - b;
    println!("Subtraction: {}", result);
}



// This code is a simple Rust program that interacts with the user, generates a random number,
// and performs basic arithmetic operations (multiplication, addition, subtraction, and division)
// with the user's input and the random number. It uses the `rand` crate for generating
// random numbers and handles user input through the standard input stream. The program includes
// functions for each arithmetic operation, which are called after the user inputs a number.
