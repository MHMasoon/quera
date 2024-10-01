use std::io::stdin;

fn main() {
    // get 4 numbers from user
    let mut numbers: [i32; 4] = [0; 4];

    for number in numbers.iter_mut() {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();
        *number = input;
    }

    // calculate sum, average, product, max, and min of four numbers
    let mut sum: i64 = 0;
    numbers.iter().for_each(|&number| sum += number as i64);

    let average: f64 = sum as f64 / 4.0;
    
    let mut product: i64 = 1;
    numbers.iter().for_each(|&number| product *= number as i64);

    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    for &number in numbers[1..].iter() {
        if number > max { max = number};
        if number < min { min = number};
    }

    // format and print results
    println!("Sum : {:.6}", sum as f64);
    println!("Average : {:.6}", average);
    println!("Product : {:.6}", product as f64);
    println!("MAX : {:.6}", max as f32);
    println!("MIN : {:.6}", min as f32);
}
