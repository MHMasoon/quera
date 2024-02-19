use std::io::stdin;

fn main() {
    // get user input
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    // extract file name and number of copy from user input
    let values: Vec<&str> = user_input.trim().split(' ').collect();
    let number_of_copy: u8 = values[0].parse().unwrap();
    let file_name: String = values[1].to_string();

    // create final string
    let mut final_string = String::new();
    for _ in 1..=number_of_copy {
        final_string.push_str("copy of ");
    }
    final_string.push_str(&file_name);

    // print final string
    println!("{final_string}");
}
