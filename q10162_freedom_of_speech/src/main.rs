use std::io::stdin;

fn main() {
    // get last curse number from user
    let mut last_curse_number = String::new();
    stdin().read_line(&mut last_curse_number).unwrap();
    let last_curse_number: u8 = last_curse_number.trim().parse().unwrap();

    // check if the number is even or odd
    if last_curse_number % 2 == 0 {
        println!("Bala Barare");
    } else {
        println!("Payin Barare");
    }
}
