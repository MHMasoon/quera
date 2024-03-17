use std::io::stdin;

fn main() {
    // get health stickers colors
    let mut colors = String::new();
    stdin().read_line(&mut colors).unwrap();
    
    // extract colors numbers
    let mut red: u8 = 0;
    let mut yellow: u8 = 0;
    let mut green: u8 = 0;

    for color in colors.chars() {
        match color {
            'R' => red += 1,
            'Y' => yellow += 1,
            'G' => green += 1,
            _ => (),
        }
    }

    // check if eating the food violates the regime
    if red >= 3 || (red >= 2 && yellow >= 2) || green == 0 {
        println!("nakhor lite");
    } else {
        println!("rahat baash");
    }
}
