use std::io::stdin;

fn main() {
    // get the rows number from user
    let mut rows_number = String::new();
    stdin().read_line(&mut rows_number).unwrap();
    let rows_number: u32 = rows_number.trim().parse().unwrap();

    // loop through rows and print multiplications
    for row in 1..=rows_number {
        let mut row_string = String::new();
        for column in 1..=rows_number {
            let multiplication: u32 = (row * column).into();
            let string_multiplication = multiplication.to_string();
            row_string.push_str(&string_multiplication);
            row_string.push(' ');
        }
        println!("{row_string}");
    }
}
