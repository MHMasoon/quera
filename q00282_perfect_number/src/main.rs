use std::io::stdin;

fn main() {
    // get the number from user
    let mut number = String::new();
    stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().unwrap();

    // calculate sum of the divisors of the number
    let mut divisors_sum: u32 = 0;
    for divisor in 1..=number / 2 {
        if number % divisor == 0 {
            divisors_sum += divisor;
        }
    }

    // check if divisors sum is equel to the number
    if number == divisors_sum {
        println!("YES");
    } else {
        println!("NO");
    }
}
