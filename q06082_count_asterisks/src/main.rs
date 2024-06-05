use std::io::stdin;
use std::io::BufRead;

fn main() {
    // get table dimensions from user
    let mut table_dimensions = String::new();
    stdin().read_line(&mut table_dimensions).unwrap();
    let mut table_dimensions_iter = table_dimensions.trim().split(' ').into_iter();
    let rows_count: u8 = table_dimensions_iter.next().unwrap().parse().unwrap();

    // count astriks of each line and store the totol
    let mut astrisks_count: u16 = 0;
    let mut lines_iter = stdin().lock().lines();

    for _ in 1..=rows_count {
        let line = lines_iter.next().unwrap().unwrap();
        for char in line.chars() {
            if char == '*' {
                astrisks_count = astrisks_count + 1;
            }
        }
    }

    // print final result
    println!("{}", astrisks_count);
}
