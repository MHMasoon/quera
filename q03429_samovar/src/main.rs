use std::io::stdin;

fn main() {
    let mut text_temp = String::new();
    stdin()
        .read_line(&mut text_temp)
        .expect("Faile to read the user input!");

    let temp: i16 = text_temp
        .trim()
        .parse()
        .expect("Faile to convert String to i16!");

    if temp < 0 {
        println!("Ice");
    } else if temp > 100 {
        println!("Steam");
    } else {
        println!("Water");
    }
}
