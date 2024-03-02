use std::io::stdin;

fn main() {
    // create free_days vector
    // note that the order is important because it should be a sorted vector
    let mut free_days = 
        vec!["1shanbe", "2shanbe", "3shanbe", "4shanbe", "5shanbe", "jome", "shanbe"];

    // get busy days of 3 friends and store it in all_busy_days vector
    let mut all_busy_days: Vec<String> = Vec::new();

    for _ in 1..=3 {
        // get number of busy days (it won't be used)
        // I read it only because of input pattern
        let mut busy_days_number = String::new();
        stdin().read_line(&mut busy_days_number).unwrap();

        // get busy days and insert into all_busy_days vector
        let mut busy_days = String::new();
        stdin().read_line(&mut busy_days).unwrap();
        let busy_days: Vec<String> = busy_days.trim().split(' ')
            .map(|day| day.to_string()).collect();
        all_busy_days.extend(busy_days);
    }

    // sort all_busy_days and remove duplicate days from it
    all_busy_days.sort();
    all_busy_days.dedup();

    // delete all_busy_days items from week_days
    for day in all_busy_days.iter() {
        let day_index_result = free_days.binary_search(&day.as_str());
        match day_index_result {
            Ok(day_index) => {
                free_days.remove(day_index);
            },
            _ => {},
        }
    }

    // print the length of free_days
    println!("{:?}", free_days.len());
}
