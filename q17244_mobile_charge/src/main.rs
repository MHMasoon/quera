use std::io::stdin;

fn main() {
    // get target_charge_percent from user
    let mut target_charge_percent = String::new();
    stdin().read_line(&mut target_charge_percent).unwrap();
    let target_charge_percent: u16 = target_charge_percent.trim().parse().unwrap();

    // calculate total minutes that is needed for charging
    let mut total_minutes: u16 = 0;
    for minutes in 0..=target_charge_percent {
        total_minutes += minutes;
    }

    // print final resutl
    println!("{total_minutes}");
}
