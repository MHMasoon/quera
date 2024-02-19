use std::io::stdin;

fn main() {
    // get row and column from user
    let mut row_and_column = String::new();
    stdin().read_line(&mut row_and_column).unwrap();
    let row_and_column: Vec<&str> = row_and_column.trim().split(' ').collect();
    let row: u8 = row_and_column[0].parse().unwrap();
    let column: u8 = row_and_column[1].parse().unwrap();

    // create dirction variable
    let mut direction = String::new();

    // check if he should go right or left
    if column <= 10 {
        direction.push_str("Right");
    } else {
        direction.push_str("Left");
    }
    direction.push(' ');

    // calculate steps to row
    let steps_to_row = (11 - row).to_string();
    direction.push_str(&steps_to_row);
    direction.push(' ');

    // calculate steps to column
    let steps_to_column: u8;
    if column > 10  {
        steps_to_column = 21 - column;
    } else {
        steps_to_column = column;
    }

    let steps_to_column = steps_to_column.to_string();
    direction.push_str(&steps_to_column);

    // print final direction
    println!("{direction}");
}
