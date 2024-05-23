use std::io::stdin;

fn main() {
    // get the word from user
    let mut word = String::new();
    stdin().read_line(&mut word).unwrap();
    word = word.trim().to_string();

    // create a loop to print echos
    for (index, letter) in word.char_indices() {
        let mut echo = String::new();
        echo.push_str(&letter.to_string().repeat(index + 1));
        echo.push_str(&word[index + 1..]);
        println!("{}", echo);
    }
}
