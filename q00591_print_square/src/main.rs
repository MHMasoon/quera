use std::io::stdin;

fn main() {
    // get the square side length from user
    let mut side_length = String::new();
    stdin().read_line(&mut side_length).unwrap();
    let side_length: u8 = side_length.trim().parse().unwrap();

    // create first and last line string 
    let mut first_and_last_rows = String::new();
    for _ in 1..=side_length {
        first_and_last_rows.push('*');
    }

    // create other lines string
    let mut other_rows = String::new();
    other_rows.push('*');
    for _ in 2..side_length {
        other_rows.push(' ');
    }
    other_rows.push('*');
    
    // print the square
    println!("{first_and_last_rows}");
    for _ in 2..side_length {
        println!("{other_rows}");
    }
    println!("{first_and_last_rows}");
}
