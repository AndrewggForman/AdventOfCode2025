use std::fs;
fn main() {
    let contents = fs::read_to_string("src/data/d5input.txt")
        .expect("Should have been able to read: src/data/d5input.txt");

    let mut id_ranges: String = String::from("");
    let mut id_availabilities: Vec<String> = Vec::new();
    let mut curr_string = String::from("");

    for char in contents.chars() {
        match char {
            '\r' => {}
            '\n' => {
                if curr_string.len() == 0 {
                    continue;
                }
                if curr_string.contains('-') {
                    id_ranges = id_ranges + &curr_string + "\n";
                    curr_string = String::from("");
                } else {
                    id_availabilities.push(curr_string);
                    curr_string = String::from("");
                }
            }
            _ => {
                curr_string.push(char);
            }
        }
    }

    println!("Ranges: {:#?}", id_ranges);
    println!("Availabilities: {:#?}", id_availabilities);

    let mut low_range: Vec<String> = Vec::new();
    let mut high_range: Vec<String> = Vec::new();
    for char in id_ranges.chars() {
        match char {
            '-' => {
                low_range.push(curr_string);
                curr_string = String::from("");
            }
            '\n' => {
                high_range.push(curr_string);
                curr_string = String::from("");
            }
            _ => curr_string.push(char),
        }
    }

    println!("Low range: {:#?}", low_range);
    println!("High range: {:#?}", high_range);

    if low_range.len() != high_range.len() {
        panic!("ERROR::Lopsided Bounds!");
    }

    let mut id_counter: usize = 0;
    let mut total_available = 0;

    for index in id_availabilities {
        let curr_id = string_to_u64(&index);
        println!("curr_id: {}", curr_id);
        let range_len = low_range.len();
        let mut iterator: usize = 0;
        while iterator < range_len {
            let low_bound = &low_range[iterator];
            let high_bound = &high_range[iterator];
            let mut low_bound = string_to_u64(low_bound);
            let high_bound = string_to_u64(high_bound);
            if curr_id >= low_bound && curr_id <= high_bound {
                println!(
                    "curr_id: {}, low_bound: {}, high_bound: {}",
                    curr_id, low_bound, high_bound
                );
                total_available += 1;
                break;
            }
            iterator += 1;
        }
    }

    println!("total_available: {}", total_available);

    // for index in [low_range.len() as usize] {
    //     let low_bound = &low_range[index as usize];
    //     let high_bound = &high_range[index as usize];
    //     let mut low_bound = string_to_u64(low_bound);
    //     let high_bound = string_to_u64(high_bound);

    //     while low_bound <= high_bound {
    //         if string_to_u64(&id_availabilities[id_counter]) >= low_bound
    //             && string_to_u64(&id_availabilities[id_counter]) <= high_bound
    //         {
    //             total_available += 1;
    //         }
    //         id_counter += 1;
    //         low_bound += 1;
    //     }
    // }
}

fn string_to_u64(nums: &String) -> u64 {
    nums.parse::<u64>().unwrap()
}

fn char_to_u64(num: char) -> u64 {
    num.to_digit(10).unwrap() as u64
}
