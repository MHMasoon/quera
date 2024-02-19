use std::io::stdin;

fn main(){
    // get a number from user
    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .expect("Failed to read number!");

    // converting input_string to number
    let number: u32 = input_string
        .trim()
        .parse()
        .expect("Failed to convert input_string to a number!");

    println!("{}", factorial(number));
}

fn factorial(mut number: u32) -> u32 {
    let mut factorial: u32 = 1;
    while number > 1 {
        factorial = factorial * number;
        number -= 1;
    };
    factorial
}
