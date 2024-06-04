use std::io::stdin;

fn main() {
    // get the number and the exponent of number 2 from user
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    let mut user_input_iter = user_input.trim().split(' ');

    let number: i64 = user_input_iter.next().unwrap().parse().unwrap();
    let exponent: u32 = user_input_iter.next().unwrap().parse().unwrap();

    // divide the number by 2 raised to the power of the exponent
    let mut division_result = number / 2_i64.pow(exponent);
    if number < 0 && number % 2_i64.pow(exponent) != 0 {
        division_result = division_result - 1;
    }

    println!("{}", division_result);
}
