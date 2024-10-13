use std::io::stdin;

fn main() {
    // get user inputs and check if they contain "MOLANA" or "HAFEZ"
    let mut passed_inputs_indexes: Vec<u8> = Vec::new();
    for index in 0..5 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.contains("MOLANA") || input.contains("HAFEZ") {
            passed_inputs_indexes.push(index + 1);
        }
    }

    // print indexes of passed inputs
    if passed_inputs_indexes.len() > 0 {
        passed_inputs_indexes.iter().for_each(|&index| print!("{} ", index));
    } else {
        println!("NOT FOUND!");
    }
}