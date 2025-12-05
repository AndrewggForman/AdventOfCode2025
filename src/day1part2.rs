use std::fs;

#[derive(PartialEq, Debug)]

enum DIRECTION {
    LEFT = -1,
    RIGHT = -2,
}

fn main() {
    println!("Hello, world!");

    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read: input.txt");

    let mut current_degree: u32 = 50;
    let mut current_direction: Option<DIRECTION> = None;
    let mut first: bool = true;
    let mut current_number_string: String = String::from("");
    let mut score: u32 = 0;

    for char in contents.chars() {
        // Very first character is used to initialized current direction to a valid option instead of None
        if first {
            match char {
                'L' => current_direction = Some(DIRECTION::LEFT),
                'R' => current_direction = Some(DIRECTION::RIGHT),
                _ => panic!("String didn't start with a direction"),
            }
            first = false;
            continue;
        }
        // Main logic post initializing direction
        match char {
            // next command will be left
            'L' => match current_direction {
                Some(DIRECTION::LEFT) => {
                    let mut results: (u32, u32);

                    results = getDegreeAndTurns(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    current_degree = results.0;
                    score += results.1;

                    // empty string of numbers & set direction
                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::LEFT)
                }
                Some(DIRECTION::RIGHT) => {
                    let mut results: (u32, u32);

                    results = getDegreeAndTurns(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    current_degree = results.0;
                    score += results.1;

                    // empty string of numbers & set direction
                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::LEFT)
                }
                _ => panic!("current_direction should only contain LEFT or RIGHT at this point!"),
            },
            // next command will be right
            'R' => match current_direction {
                Some(DIRECTION::LEFT) => {
                    let mut results: (u32, u32);

                    results = getDegreeAndTurns(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    current_degree = results.0;
                    score += results.1;

                    // empty string of numbers & set direction
                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::RIGHT)
                }
                Some(DIRECTION::RIGHT) => {
                    let mut results: (u32, u32);

                    results = getDegreeAndTurns(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    current_degree = results.0;
                    score += results.1;

                    // empty string of numbers & set direction
                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::RIGHT)
                }
                _ => panic!("current_direction should only contain LEFT or RIGHT at this point!"),
            },
            // what digit to add
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                current_number_string.push(char)
            }
            // skim over newspace characters/trash characters
            _ => {}
        }
    }

    // Catches last turn not caught by while loop
    let mut results: (u32, u32);

    results = getDegreeAndTurns(
        stringToU32(current_number_string),
        current_direction,
        current_degree,
    );

    current_degree = results.0;
    score += results.1;

    current_number_string = String::from("");
    current_direction = None;

    println!(
        "FINALE\nCURRENT DEGREE: {}, SCORE: {}",
        current_degree, score
    )
}

// returns turns & new current degree
fn getDegreeAndTurns(
    mut input_degree: u32,
    direction: Option<DIRECTION>,
    mut current_degree: u32,
) -> (u32, u32) {
    // for debugging purposes
    println!(
        "CURRENT DEGREE: {}, INPUT DEGREE: {}, CURRENT DIRECTION: {:#?}",
        current_degree, input_degree, direction
    );

    let mut zero_touches: u32 = 0;
    let started_at_zero: bool = if current_degree == 0 { true } else { false };
    match direction {
        Some(DIRECTION::LEFT) => {
            // if starting at 0, take a turn to the left without incrementing zero_touches
            if (current_degree < input_degree) && (started_at_zero) {
                input_degree = input_degree - current_degree;
                current_degree = 100;
            }

            // if input_degree - current degree == 0 without looping, incrementing zero touch
            if (current_degree > 0 && input_degree < 100) && (current_degree == input_degree) {
                zero_touches += 1;
            }

            while (current_degree < input_degree) {
                input_degree = input_degree - current_degree;
                current_degree = 100;
                zero_touches += 1;
            }

            // if remaining input_degree < 100, don't increment, get new current degree
            // if 100, increment zero touches before setting current_degree to 0
            if input_degree == 100 {
                zero_touches += 1;
            }
            current_degree = current_degree - input_degree;
        }
        Some(DIRECTION::RIGHT) => {
            while (current_degree + input_degree) > 99 {
                input_degree = input_degree + current_degree - 100;
                current_degree = 0;
                zero_touches += 1;
            }
            current_degree = input_degree + current_degree
        }
        _ => panic!("Illegal direction"),
    }

    // for debugging purposes
    println!(
        "NEW DEGREE: {}, ZERO TOUCHES: {}\n\n",
        current_degree, zero_touches
    );

    (current_degree, zero_touches)
}

// one liner for turning strings to u32 integer
fn stringToU32(nums: String) -> u32 {
    nums.parse::<u32>().unwrap()
}
