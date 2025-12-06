use std::fs;
fn main() {
    let contents = fs::read_to_string("src/data/d3input.txt")
        .expect("Should have been able to read: src/data/d3input.txt");

    let mut total_score: u64 = 0;
    let mut battery_pack: String = String::from("");

    // Loop through battery pack
    for char in contents.chars() {
        match char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                battery_pack.push(char)
            }
            ',' => {
                println!("battery_pack: {}", battery_pack);
                total_score += batteries_to_voltage(battery_pack);
                battery_pack = String::from("")
            }
            _ => {}
        }
    }

    // Catches tail end
    println!("battery_pack: {}", battery_pack);
    total_score += batteries_to_voltage(battery_pack);

    println!("total_score: {}", total_score);
}

fn string_to_u64(nums: String) -> u64 {
    nums.parse::<u64>().unwrap()
}

fn batteries_to_voltage(battery_pack: String) -> u64 {
    // TODO

    // Vector of batteres
    let mut batteries: Vec<char> = Vec::new();
    for battery in battery_pack.chars() {
        batteries.push(battery);
    }

    return 10;
}

fn find_first_battery_and_index(batteries: &Vec<char>) -> (char, u64) {
    ('c', 10)
}
