use std::io::stdin;

fn main() {
    // get the diameter of the rhumbuses from user
    let mut diameter = String::new();
    stdin().read_line(&mut diameter).unwrap();
    let diameter: usize = diameter.trim().parse().unwrap();

    let middle: usize = diameter / 2 + 1;
    for row in 1..=diameter {
        let mut row_vector = vec![' '; diameter * 2];
        let distance_from_center: usize = middle.abs_diff(row);
        for index in distance_from_center..diameter - distance_from_center {
            row_vector[index] = '*';
            row_vector[index + diameter] = '*';
        }
        // convert row vector to string for the purpose of printing
        let row_string: String = row_vector.iter().collect();
        println!("{}", row_string);
    }
}
