use std::io::stdin;

fn main() {
    // get the number from user
    let mut number = String::new();
    stdin().read_line(&mut number).unwrap();
    let number = number.trim();

    // loop through number digits
    for string_digit in number.chars() {
        let int_digit: u8 = string_digit.to_digit(10).unwrap() as u8;
        let mut digit_row = String::new();
        digit_row.push(string_digit);
        digit_row.push_str(": ");
        for _ in 1..=int_digit {
            digit_row.push(string_digit);
        }
        println!("{digit_row}");
    }
}
