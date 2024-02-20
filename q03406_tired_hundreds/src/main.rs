use std::io::stdin;

fn main() {
    // get two numbers from user
    let mut num1 = String::new();
    stdin().read_line(&mut num1).unwrap();
    num1 = num1.trim().to_string();
    let reversed_num1: String = num1.chars().rev().collect();

    let mut num2 = String::new();
    stdin().read_line(&mut num2).unwrap();
    num2 = num2.trim().to_string();
    let reversed_num2: String = num2.chars().rev().collect();

    // compare two numbers as string
    if reversed_num1 == reversed_num2 {
        println!("{num2} = {num1}");
    } else if reversed_num1 > reversed_num2 {
        println!("{num2} < {num1}");
    } else {
        println!("{num1} < {num2}");
    }
}
