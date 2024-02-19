use std::io::stdin;

fn main() {
    let mut text_number = String::new();
    stdin()
        .read_line(&mut text_number)
        .expect("Failed to read text!");

    let number: u8;
    number = text_number
        .trim()
        .parse()
        .expect("Failed to convert text to number!");
    
    let wow = wow(number);
    println!("{}", wow);
}

fn wow(mut level: u8) -> String {
    let mut wow = String::from("W");
    while level >= 1 {
        wow.push('o');
        level -= 1;
    }
    wow.push_str("w!");
    wow
}