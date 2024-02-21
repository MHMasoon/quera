use std::io::stdin;

fn main() {
    // get birth_date from user
    let mut birth_date = String::new();
    stdin().read_line(&mut birth_date).unwrap();
    let birth_date = birth_date.trim();

    // extract and print birth year and birth month
    println!("saal:{}", &birth_date[0..=1]);
    println!("maah:{}", &birth_date[2..=3]);
}
