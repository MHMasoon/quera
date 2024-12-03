use std::io::stdin;

fn main() {
    // Get a number from user
    let mut number = String::new();
    stdin().read_line(&mut number).unwrap();
    let number = number.trim();
    
    // Print final result
    println!("Hello CodeCup {}!", number);
}
