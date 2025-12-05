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
            'L' => match current_direction {
                Some(DIRECTION::LEFT) => {
                    current_degree = getNewDegree(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    if current_degree == 0 {
                        score += 1;
                    }

                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::LEFT)
                }
                Some(DIRECTION::RIGHT) => {
                    current_degree = getNewDegree(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    if current_degree == 0 {
                        score += 1;
                    }

                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::LEFT)
                }
                _ => panic!("current_direction should only contain LEFT or RIGHT at this point!"),
            },
            'R' => match current_direction {
                Some(DIRECTION::LEFT) => {
                    current_degree = getNewDegree(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    if current_degree == 0 {
                        score += 1;
                    }

                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::RIGHT)
                }
                Some(DIRECTION::RIGHT) => {
                    current_degree = getNewDegree(
                        stringToU32(current_number_string),
                        current_direction,
                        current_degree,
                    );

                    if current_degree == 0 {
                        score += 1;
                    }

                    current_number_string = String::from("");
                    current_direction = Some(DIRECTION::RIGHT)
                }
                _ => panic!("current_direction should only contain LEFT or RIGHT at this point!"),
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                current_number_string.push(char)
            }
            _ => println!("newline / trash character"),
        }
    }

    // Catches last turn
    current_degree = getNewDegree(
        stringToU32(current_number_string),
        current_direction,
        current_degree,
    );

    if current_degree == 0 {
        score += 1;
    }

    current_number_string = String::from("");
    current_direction = None;

    println!("CURRENT DEGREE: {}, SCORE: {}", current_degree, score)
}

fn getNewDegree(mut degree: u32, direction: Option<DIRECTION>, mut current_degree: u32) -> u32 {
    println!(
        "CURRENT DEGREE: {}, INPUT DEGREE: {}, CURRENT DIRECTION: {:#?}",
        current_degree, degree, direction
    );
    match direction {
        Some(DIRECTION::LEFT) => {
            while current_degree < degree {
                degree = degree - current_degree;
                current_degree = 100;
            }
            current_degree = current_degree - degree
        }
        Some(DIRECTION::RIGHT) => {
            while (current_degree + degree) > 99 {
                degree = degree + current_degree - 100;
                current_degree = 0
            }
            current_degree = degree + current_degree
        }
        _ => panic!("Illegal direction"),
    }
    println!("NEW DEGREE: {}", current_degree);
    current_degree
}

fn stringToU32(nums: String) -> u32 {
    nums.parse::<u32>().unwrap()
}
