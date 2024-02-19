use std::io::stdin;

fn main() {
    // get brays number and intervals from user
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    
    // extract values from user_input
    let values: Vec<&str> = user_input.trim().split(' ').collect();
    let odd_interval: u32 = values[0].parse().unwrap();
    let even_interval: u32 = values[1].parse().unwrap();
    let brays_number: u32 = values[2].parse().unwrap();

    // calculate bray duration
    let mut bray_duration: u32;
    bray_duration = (brays_number / 2) * (odd_interval + even_interval);
    if brays_number % 2 == 1 {
        bray_duration += odd_interval;
    }

    // print final resutl
    println!("{bray_duration}");
}
