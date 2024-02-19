use std::io::stdin;

fn main() {
    // get the numbers and store them until it's a zero
    let mut numbers: Vec<String> = Vec::new();
    loop {
        let mut number = String::new();
        stdin().read_line(&mut number).unwrap();
        number = number.trim().to_string();
        
        if number == "0" {
            break;
        } else {
           numbers.push(number);
        }
    }

    // reverse and print stored numbers
    reverse_print_vector(numbers);
}

// reverse a vector and print the items
fn reverse_print_vector(mut numbers: Vec<String>) {
    numbers.reverse();
    for number in numbers.iter() {
        println!("{number}");
    }
}