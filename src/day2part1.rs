use std::fs;
fn main() {
    let contents = fs::read_to_string("src/data/d2p1input.txt")
        .expect("Should have been able to read: src/data/d2p1input.txt");

    let mut total_score: u64 = 0;
    let mut current_range: String = String::from("");
    for char in contents.chars() {
        match char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' => {
                current_range.push(char)
            }
            ',' => {
                println!("Current Range: {}", current_range);
                total_score += rangeToRepeats(current_range);
                current_range = String::from("")
            }
            _ => {}
        }
    }

    // Catches tail range
    println!("Current Range: {}", current_range);
    total_score += rangeToRepeats(current_range);

    println!("Repeats: {}", total_score);
}

fn repeats_twice(num: String) -> bool {
    // To repeat it has to be even.
    if (num.len() % 2) == 1 {
        return false;
    }

    // Store chars in a vector
    let mut digits: Vec<char> = Vec::new();
    for char in num.chars() {
        digits.push(char);
    }

    // Setup iterating variables
    let mut i: u64 = 0;
    let length: u64 = num.len().try_into().unwrap();
    let half_length: u64 = length / 2;

    // Check if first half of word matches second half of word
    while i < half_length {
        if digits[i as usize] != digits[(i + half_length) as usize] {
            return false;
        }
        i += 1;
    }

    return true;
}

fn rangeToRepeats(range: String) -> u64 {
    // Get two nums (as strings) from a range like: "333-355"
    let mut low_num_str: String = String::from("");
    let mut high_num_str: String = String::from("");
    let mut finished_low_num: bool = false;

    for char in range.chars() {
        match char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if finished_low_num {
                    high_num_str.push(char);
                } else {
                    low_num_str.push(char);
                }
            }
            '-' => finished_low_num = true,
            _ => {}
        }
    }

    // Iterate through the range of numbers: 333 to 355
    let mut sum_of_repeats: u64 = 0;
    let low_num: u64 = stringToU64(low_num_str);
    let high_num: u64 = stringToU64(high_num_str);
    let mut curr_num: u64 = low_num;
    while curr_num <= high_num {
        if repeats_twice(curr_num.to_string()) {
            sum_of_repeats += curr_num;
        }
        curr_num += 1;
    }

    return sum_of_repeats;
}

fn stringToU64(nums: String) -> u64 {
    nums.parse::<u64>().unwrap()
}
