use std::io::stdin;

fn main() {
    // get letters_number, target_word and user_word from user
    let mut letters_number = String::new();
    stdin().read_line(&mut letters_number).unwrap();
    let letters_number: usize = letters_number.trim().parse().unwrap();

    let mut target_word = String::new();
    stdin().read_line(&mut target_word).unwrap();
    
    let mut user_word = String::new();
    stdin().read_line(&mut user_word).unwrap();

    // compare target_word and user_word and count the errors
    let mut errors_number: u32 = 0;
    for letter_index in 0..letters_number {
        if target_word.as_bytes()[letter_index] != user_word.as_bytes()[letter_index] {
            errors_number += 1;
        }
    }

    // print errors_number
    println!("{errors_number}");
}
