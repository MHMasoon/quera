use std::io::stdin;

fn main() {
    // Get user input
    let mut angles_string = String::new();
    stdin().read_line(&mut angles_string).expect("Failed to read your input!");

    // Parse user input into a tuple
    let mut angles: (u16, u16, u16) = (0, 0, 0);
    let mut iter = angles_string.splitn(3, ' ');
    angles.0 = iter.next().unwrap().trim().parse().expect("You must enter three numbers!");
    angles.1 = iter.next().unwrap().trim().parse().expect("You must enter three numbers!");
    angles.2 = iter.next().unwrap().trim().parse().expect("You must enter three numbers!");

    // Check if angles can form a triangle
    if angles.0 + angles.1 + angles.2 != 180 {
        println!("No");
    } else if angles.0 == 0 || angles.1 == 0 || angles.2 == 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
