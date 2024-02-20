use std::io::stdin;

fn main() {
    // get the numbers and number of them from user
    let mut _numbers_number = String::new();
    stdin().read_line(&mut _numbers_number).unwrap();

    let mut numbers = String::new();
    stdin().read_line(&mut numbers).unwrap();

    // convert numbers into a vector
    let numbers: Vec<u16> = numbers
        .trim().split(' ')
        .map(|number| number.parse().unwrap())
        .collect();

    // find maximum number and print it
    let maximum_number = numbers.iter().max().unwrap();
    println!("{maximum_number}");
}
