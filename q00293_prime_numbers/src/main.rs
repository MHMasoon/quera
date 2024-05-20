use std::io::stdin;

// create a function to check if a number is prime
pub fn is_prime(number: u16) -> bool {
    if number == 1 {
        return false;
    }

    let half: u16 = number / 2;
    for divisor in 2..=half {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

fn main() {
    // get first_number and last_number from user
    let mut first_number = String::new();
    stdin().read_line(&mut first_number).unwrap();
    let first_number: u16 = first_number.trim().parse().unwrap();

    let mut last_number = String::new();
    stdin().read_line(&mut last_number).unwrap();
    let last_number: u16 = last_number.trim().parse().unwrap();

    // loop through numbers starting from first_number and ending to last_number
    for number in first_number..=last_number {
        if is_prime(number) {
            println!("{number}");
        }
    }
}
