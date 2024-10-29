use std::io::stdin;
fn main() {
    // Take count of numbers and numbers
    let mut count = String::new();
    let mut numbers = String::new();
    stdin().read_line(&mut count).unwrap();
    stdin().read_line(&mut numbers).unwrap();

    // Convert numbers string to vector
    let mut biggest_number = 0_u8;
    let mut biggest_number_index = 0_usize;
    numbers.trim().split(' ').map(|number| number.parse::<u8>().unwrap()).enumerate().for_each(|(index, number)| {
        if number > biggest_number {
            biggest_number = number;
            biggest_number_index = index;
        }
    });

    println!("{}", biggest_number_index + 1);
}
