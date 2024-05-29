use std::io::stdin;

fn main() {
    // get user's weight and height
    let mut weight = String::new();
    let mut height = String::new();

    stdin().read_line(&mut weight).unwrap();
    stdin().read_line(&mut height).unwrap();

    let weight: f32 = weight.trim().parse().unwrap();
    let height: f32 = height.trim().parse().unwrap();

    // calculate BMI number
    let bmi_number: f32 = weight / (height * height);
    
    // determine BMI class
    let bmi_class = if bmi_number < 18.5 {
        "Underweight".to_string()
    } else if 18.5 <= bmi_number && bmi_number < 25.0 {
        "Normal".to_string()
    } else if 25.0 <= bmi_number && bmi_number < 30.0 {
        "Overweight".to_string()
    } else {
        "Obese".to_string()
    };

    // print final results
    println!("{:.2}", bmi_number);
    println!("{}", bmi_class);
}
