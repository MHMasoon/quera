use std::io::stdin;

fn calculate_digits_sum(string_number: String) -> String {
    let mut digits_sum: u8 = 0;
    for string_digit in string_number.trim().chars() {
        digits_sum += string_digit.to_digit(10).unwrap() as u8;
    }
    digits_sum.to_string()
}

fn main() {
    // get a number from user
    let mut string_number = String::new();
    stdin().read_line(&mut string_number).unwrap();

    // calculate digits sum until it's lower than 10
    let mut digits_sum = calculate_digits_sum(string_number);
    while digits_sum.len() > 1 {
        digits_sum = calculate_digits_sum(digits_sum);
    }
    println!("{digits_sum}");
}
