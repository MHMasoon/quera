use std::io::stdin;

fn main() {
    let mut text_number = String::new();

    stdin()
        .read_line(&mut text_number)
        .expect("Faile to read user input!");

    let mut number: u8 = text_number
        .trim()
        .parse()
        .expect("Failed to convert String to u8");

    while number > 0 {
        println!("man khoshghlab hastam");
        number -= 1;
    }
}
