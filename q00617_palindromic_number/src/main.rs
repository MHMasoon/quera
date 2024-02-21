use std::io::stdin;

fn main() {
    // get a number from user
    let mut number = String::new();
    stdin().read_line(&mut number).unwrap();
    number = number.trim().to_string();

    // reverse the number
    let reversed_number: String = number.chars().rev().collect();

    // compare the number and its reversed version
    if number == reversed_number {
        println!("YES");
    } else {
        println!("NO");
    }
}
