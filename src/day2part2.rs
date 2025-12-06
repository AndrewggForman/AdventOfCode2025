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
                total_score += range_to_repeats(current_range);
                current_range = String::from("")
            }
            _ => {}
        }
    }

    // Catches tail range
    println!("Current Range: {}", current_range);
    total_score += range_to_repeats(current_range);

    println!("Repeats: {}", total_score);
}

// Looks for any length repeat
fn any_repeats(num: String) -> bool {
    // Store digits in a vector
    let mut digits: Vec<char> = Vec::new();
    for char in num.chars() {
        digits.push(char);
    }

    let length: u64 = num.len().try_into().unwrap();
    let half_length: u64 = length / 2;
    let mut i: u64 = half_length;

    // Index from half to 1, catching all divisors.
    // e.g. if length is 12, check splitting it into 6, then 4, then 3, then 2, then 1 length blocks

    while i > 0 {
        // Skip if it cannot be divided by the number cleanly
        // E.g. if length is 12, cannot cleanly be split into 5-long chunks
        if is_repeatable(length, i) {
            if check_repeats(length, i, &digits) {
                return true;
            }
        }
        i -= 1;
    }

    // Didn't find a repeat, end by returning false
    return false;
}

fn check_repeats(total_length: u64, sub_length: u64, digits: &Vec<char>) -> bool {
    let mut base_word: String = String::from("");
    let mut curr_word: String = String::from("");
    let mut i: u64 = 0;

    // build subword e.g. "1241_4444_4321" has a length of 12, if we're checking length of 4 atm
    // then build 1241
    while i < sub_length {
        base_word.push(digits[i as usize]);
        i += 1;
    }

    i = 0;
    let mut this_loop: u64 = 0;
    let mut overall_index: u64 = 0;

    // Basically, build x amount of sub words by iterating through vector,
    // return false if any of the sub words are not equal, if all are equal then return true
    // E.g. "1221_1221_1221" will build 1221 as the base, then check the first 1221, the second 1221, and the third 1221
    // E.g. of a failure -> "4444_4444_4443" -> check 4444, passes. check 4444, passes. check 4443, fails.
    while this_loop * sub_length < total_length {
        while i < sub_length {
            curr_word.push(digits[overall_index as usize]);
            i += 1;
            overall_index += 1;
        }
        this_loop += 1;
        i = 0;
        if base_word != curr_word {
            //println!("base: {}, curr: {}", base_word, curr_word);
            return false;
        }
        curr_word = String::from("");
    }
    return true;
}

fn is_repeatable(length: u64, divisor: u64) -> bool {
    return length % divisor == 0;
}

// Converts from a range "333-355" to the sum of its repeats
fn range_to_repeats(range: String) -> u64 {
    // Get two nums (as strings) from a range like: "333-355"
    let mut low_num_str: String = String::from("");
    let mut high_num_str: String = String::from("");
    let mut finished_low_num: bool = false;

    // Build your high and low numbers as strings e,g, "333" and "355"
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
    let low_num: u64 = string_to_u64(low_num_str);
    let high_num: u64 = string_to_u64(high_num_str);
    let mut curr_num: u64 = low_num;

    while curr_num <= high_num {
        if any_repeats(curr_num.to_string()) {
            sum_of_repeats += curr_num;
        }
        curr_num += 1;
    }

    return sum_of_repeats;
}

fn string_to_u64(nums: String) -> u64 {
    nums.parse::<u64>().unwrap()
}
