use std::io::stdin;
use std::mem::swap;

fn main() {
    // This code generates a string where each character represents a natural number starting from 1.
    // If the number is part of the Fibonacci sequence, the character is '+'.
    // If the number is not part of the Fibonacci sequence, the character is '-'.
    // -------------------------------------------------------------------------------------------------

    // get fibonacci string length from user
    let mut fibonacci_string_length = String::new();
    stdin().read_line(&mut fibonacci_string_length).unwrap();
    let fibonacci_string_length: usize = fibonacci_string_length.trim().parse().unwrap();

    // create a char vector and initiate it with '-'
    let mut fibonacci_vec: Vec<char> = vec!['-'; fibonacci_string_length];

    // calculate fibonacci sequence and change respective chars to '+' 
    let mut first_number: usize = 1;
    let mut second_number: usize = 2;

    while first_number <= fibonacci_string_length {
        fibonacci_vec[first_number - 1] = '+';
        swap(&mut first_number, &mut second_number);
        second_number = second_number + first_number;
    }

    // convert fibonacci vector to string and print it
    let fibonacci_string: String = fibonacci_vec.into_iter().collect();
    println!("{}", fibonacci_string);
}
