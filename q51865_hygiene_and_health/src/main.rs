use std::io::stdin;

fn main() {
    // get grade and trip days number from user
    let (mut grade, mut trip_length) = (String::new(), String::new());
    stdin().read_line(&mut grade).unwrap();
    stdin().read_line(&mut trip_length).unwrap();

    // convert input strings to numbers
    let mut grade: u8 = grade.trim().parse().unwrap();
    let trip_length: u8 = trip_length.trim().parse().unwrap();

    // calculate the final grade
    if trip_length == 0 {
        grade = 20;
    } else if trip_length != 7 {
        if trip_length >= grade {
            grade = 0;
        } else {
            grade = grade - trip_length
        }
    }

    // print final grade
    println!("{grade}");
}
