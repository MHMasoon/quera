use std::io::stdin;

fn main() {
    // note: we call an object that starts with persian S a siin

    // create an array of siins
    let siins = ["sabzeh", "samanu", "senjed", "seer", "seeb", "somaagh", "serkeh"];

    // get the number of siins needed from user
    let mut siins_needed_number = String::new();
    stdin().read_line(&mut siins_needed_number).unwrap();
    let siins_needed_number: usize = siins_needed_number.trim().parse().unwrap();

    // print siins as much as needed
    for index in 0..siins_needed_number {
        println!("{}", siins[index]);
    }
}
