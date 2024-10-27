use std::io::stdin;
fn main() {
    // Get endpoints of the open interval
    let mut lower_bound = String::new();
    let mut upper_bound = String::new();
    stdin().read_line(&mut lower_bound).unwrap();
    let lower_bound: u16 = lower_bound.trim().parse().unwrap();
    stdin().read_line(&mut upper_bound).unwrap();
    let upper_bound: u16 = upper_bound.trim().parse().unwrap();

    // Find prime numbers in the interval
    let mut prime_numbers: Vec<String> = Vec::new();
    for number in (lower_bound + 1)..upper_bound {
        let mut is_prime: bool = true;
        for divisor in 2..=(number / 2) {
            if number % divisor == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_numbers.push(number.to_string());
        }
    }

    // Print the interval
    println!("{}", prime_numbers.join(","));
}
