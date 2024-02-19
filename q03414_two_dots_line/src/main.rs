use std::io::stdin;

fn main() {
    // get coordinates from user
    let mut coordinates = String::new();
    stdin().read_line(&mut coordinates).unwrap();

    // destruct vector of coordinates
    let mut coordinates: Vec<&str> = coordinates.trim().split(' ').collect();
    let mostafa_x = coordinates.remove(0);
    let mostafa_y = coordinates.remove(0);
    let boss_x = coordinates.remove(0);
    let boss_y = coordinates.remove(0);

    // compare mostafa coordinates whit his boss coordinates
    if mostafa_x == boss_x {
        println!("Vertical");
    } else if mostafa_y == boss_y {
        println!("Horizontal");
    } else {
        println!("Try again");
    }

}
