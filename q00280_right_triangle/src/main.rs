use std::io::stdin;

fn main() {
    // define an array to hold sides
    let mut sides: [u16; 3] = [0; 3];

    // get three sides from user
    // println!("Please insert the sides' lengths in separate lines:");
    let mut user_input: String = String::new();
    // first side
    stdin().read_line(&mut user_input).expect("Failed to read the number!");
    sides[0] = user_input.trim().parse::<u16>().unwrap();
    user_input.clear();
    // second side
    stdin().read_line(&mut user_input).expect("Failed to read the number!");
    sides[1] = user_input.trim().parse::<u16>().unwrap();
    user_input.clear();
    // third side
    stdin().read_line(&mut user_input).expect("Failed to read the number!");
    sides[2] = user_input.trim().parse::<u16>().unwrap();

    // Sort array to find the longest side
    sides.sort();

    // Check the Paythagorean theorem
    if sides[0].pow(2) + sides[1].pow(2) == sides[2].pow(2) {
        println!("YES");
    } else {
        println!("NO");
    }
}

