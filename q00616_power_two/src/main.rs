use std::io::stdin;

fn main() {
    // get the number from user
    let mut number = String::new();
    stdin().read_line(&mut number).unwrap();
    let number: u32 = number.trim().parse().unwrap();

    // find the power of 2 that is higher than the number
    for exponent in 1..=30 {
        let power_result: u32 = 2u32.pow(exponent);
        if power_result > number {
            println!("{power_result}");
            break;
        }
    }
}
