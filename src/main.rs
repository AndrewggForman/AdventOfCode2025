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
            '\n' => {
                println!("battery_pack: {}", battery_pack);
                total_score += batteries_to_voltage(battery_pack);
                battery_pack = String::from("")
            }
            _ => {}
        }
    }

    // Catches tail end

    // TODO FIX
    //total_score += batteries_to_voltage(battery_pack);
    println!("battery_pack: {}", battery_pack);

    println!("total_score: {}", total_score);
}

fn string_to_u64(nums: String) -> u64 {
    nums.parse::<u64>().unwrap()
}

fn char_to_u64(num: char) -> u64 {
    num.to_digit(10).unwrap() as u64
}

fn batteries_to_voltage(battery_pack: String) -> u64 {
    let mut total_voltage: String = String::from("");

    // Vector of batteres
    let mut batteries: Vec<char> = Vec::new();
    for battery in battery_pack.chars() {
        batteries.push(battery);
    }

    let mut iterator = 0;
    let mut starting_index = 0;

    while iterator < 12 {
        // println!(
        //     "BEFORE PUSH: starting_index: {}, total_voltage: {}, iterator: {}",
        //     starting_index, total_voltage, iterator
        // );

        let (left_hand_voltage, new_index) =
            find_voltage_and_index_from_index(&batteries, starting_index, 12 - iterator);

        total_voltage.push(left_hand_voltage);
        starting_index = new_index;
        iterator += 1;

        println!(
            "AFTER PUSH: starting_index: {}, total_voltage: {}, iterator: {}",
            starting_index,
            total_voltage,
            12 - iterator
        );
    }

    //let second_voltage = find_last_voltage(&batteries, starting_index + 1);

    println!("battery voltage: {}", total_voltage);

    return string_to_u64(total_voltage);
}

fn find_voltage_and_index_from_index(
    batteries: &Vec<char>,
    starting_index: u64,
    ending_index: u64,
) -> (char, u64) {
    let mut current_voltage: char = '0';
    let mut current_voltage_index = starting_index;
    let total_battery_count = batteries.len() as u64;
    let mut iterator_index = starting_index;

    while iterator_index <= total_battery_count - ending_index {
        if char_to_u64(batteries[iterator_index as usize]) > char_to_u64(current_voltage) {
            current_voltage = batteries[iterator_index as usize];
            current_voltage_index = iterator_index;
        }
        iterator_index += 1;
    }
    (current_voltage, current_voltage_index + 1)
}

// fn find_last_voltage(batteries: &Vec<char>, starting_index: u64) -> char {
//     let battery_pack_size: u64 = (batteries.len()).try_into().unwrap();
//     let mut iterator_index = starting_index;
//     let mut second_voltage: char = '0';

//     while iterator_index < battery_pack_size {
//         if char_to_u64(batteries[iterator_index as usize]) > char_to_u64(second_voltage) {
//             second_voltage = batteries[iterator_index as usize];
//         }
//         iterator_index += 1;
//     }
//     second_voltage
// }

// Get the largest voltage battery from the battery pack (excluding the very last battery)
// fn find_voltage_and_index(batteries: &Vec<char>) -> (char, u64) {
//     let mut battery_index = 0;
//     let mut iterator_index = 0;
//     let mut voltage: char = '0';
//     let battery_pack_size: u64 = (batteries.len()).try_into().unwrap();

//     // Iterate through battery pack (exclude very last battery)
//     while iterator_index < battery_pack_size - 1 {
//         if char_to_u64(batteries[iterator_index as usize]) > char_to_u64(voltage) {
//             voltage = batteries[iterator_index as usize];
//             battery_index = iterator_index;
//         }

//         iterator_index += 1;
//     }

//     (voltage, battery_index)
// }
