use std::io::stdin;

fn main() {
    // get the number of pieces from user
    let mut pieces_numbers = String::new();
    stdin().read_line(&mut pieces_numbers).unwrap();

    // extract each piece nubmer from user_input
    let pieces_numbers: Vec<&str> = pieces_numbers.trim().split(' ').collect();
    let king: i8 = pieces_numbers[0].parse().unwrap();
    let queen: i8 = pieces_numbers[1].parse().unwrap();
    let rook: i8 = pieces_numbers[2].parse().unwrap();
    let bishop: i8 = pieces_numbers[3].parse().unwrap();
    let knight: i8 = pieces_numbers[4].parse().unwrap();
    let pawn: i8 = pieces_numbers[5].parse().unwrap();

    // calculate the difference of what we have and what we need
    let mut pieces_needed_numbers = String::new();
    pieces_needed_numbers.push_str(&(1 - king).to_string());
    pieces_needed_numbers.push(' ');
    pieces_needed_numbers.push_str(&(1 - queen).to_string());
    pieces_needed_numbers.push(' ');
    pieces_needed_numbers.push_str(&(2 - rook).to_string());
    pieces_needed_numbers.push(' ');
    pieces_needed_numbers.push_str(&(2 - bishop).to_string());
    pieces_needed_numbers.push(' ');
    pieces_needed_numbers.push_str(&(2 - knight).to_string());
    pieces_needed_numbers.push(' ');
    pieces_needed_numbers.push_str(&(8 - pawn).to_string());

    // print final result
    println!("{pieces_needed_numbers}");
}

