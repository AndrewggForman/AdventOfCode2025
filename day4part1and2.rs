use std::fs;
fn main() {
    let contents = fs::read_to_string("src/data/d4input.txt")
        .expect("Should have been able to read: src/data/d4input.txt");

    let mut rows: Vec<Vec<char>> = Vec::new();
    let mut columns: Vec<char> = Vec::new();
    let mut total_ats = 0;

    for char in contents.chars() {
        match char {
            '@' | '.' => {
                columns.push(char);
                total_ats += 1;
            }
            '\n' => {
                rows.push(columns);
                columns = Vec::new();
            }
            _ => {}
        }
    }

    // Second vector for copying to
    let mut copy_rows = rows.clone();

    let row_size: usize = rows.len();
    let mut adjacent_paper_count: u32 = 0;
    let mut accessable_paper_count: u32 = 0;

    // Paper counts to compare for knowing when to end loop
    let mut original_paper_count: u32 = 0;
    let mut copy_paper_count: u32 = 0;
    let mut paper_removed: u32 = 0;
    let mut last_loop_paper_count: u32 = 0;

    // Loop -> Every time we find an accessable paper, update copy vector with a '.' instead of a '@'
    // & update how many paper we've removed. At the end of an iteration we break if our clone didn't change at all
    // Meaning that we've finally removed all removable papers, and nothing changed between loop iterations
    loop {
        rows = copy_rows.clone();
        last_loop_paper_count = copy_paper_count;
        original_paper_count = 0;
        copy_paper_count = 0;

        for (row_pos, row) in rows.iter().enumerate() {
            let column_size: usize = row.len();
            for (column_pos, column) in row.iter().enumerate() {
                // Handles Not-paper
                if is_empty_spot(column) {
                    copy_rows[row_pos][column_pos] = '.';
                    continue;
                } else {
                    original_paper_count += 1;
                }

                // Handles Corners
                if is_top_left(row_pos, column_pos) {
                    let mut current_row = row_pos;
                    while current_row <= row_pos + 1 {
                        let mut current_column = column_pos;
                        while current_column <= column_pos + 1 {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_top_right(row_pos, column_pos, column_size) {
                    let mut current_row = row_pos;
                    while current_row <= row_pos + 1 {
                        let mut current_column = column_pos - 1;
                        while current_column <= column_pos {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_bottom_left(row_pos, column_pos, row_size) {
                    let mut current_row = row_pos - 1;
                    while current_row <= row_pos {
                        let mut current_column = column_pos;
                        while current_column <= column_pos + 1 {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_bottom_right(row_pos, column_pos, row_size, column_size) {
                    let mut current_row = row_pos - 1;
                    while current_row <= row_pos {
                        let mut current_column = column_pos - 1;
                        while current_column <= column_pos {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                // Handles Edges
                if is_top_edge(row_pos) {
                    let mut current_row = row_pos;
                    while current_row <= row_pos + 1 {
                        let mut current_column = column_pos - 1;
                        while current_column <= column_pos + 1 {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_bottom_edge(row_pos, row_size) {
                    let mut current_row = row_pos - 1;
                    while current_row <= row_pos {
                        let mut current_column = column_pos - 1;
                        while current_column <= column_pos + 1 {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1;
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_right_edge(column_pos, column_size) {
                    let mut current_row = row_pos - 1;
                    while current_row <= row_pos + 1 {
                        let mut current_column = column_pos - 1;
                        while current_column <= column_pos {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                if is_left_edge(column_pos) {
                    let mut current_row = row_pos - 1;
                    while current_row <= row_pos + 1 {
                        let mut current_column = column_pos;
                        while current_column <= column_pos + 1 {
                            match rows[current_row][current_column] {
                                '@' => adjacent_paper_count += 1,
                                _ => {}
                            }
                            current_column += 1;
                        }
                        current_row += 1;
                    }

                    // Offsets counting self
                    adjacent_paper_count -= 1;
                    if adjacent_paper_count < 4 {
                        copy_rows[row_pos][column_pos] = '.';
                        accessable_paper_count += 1
                    }

                    adjacent_paper_count = 0;
                    continue;
                }

                // Normal Position
                let mut current_row = row_pos - 1;
                while current_row <= row_pos + 1 {
                    let mut current_column = column_pos - 1;
                    while current_column <= column_pos + 1 {
                        match rows[current_row][current_column] {
                            '@' => adjacent_paper_count += 1,
                            _ => {}
                        }
                        current_column += 1;
                    }
                    current_row += 1;
                }

                // Offsets counting self
                adjacent_paper_count -= 1;
                if adjacent_paper_count < 4 {
                    copy_rows[row_pos][column_pos] = '.';
                    accessable_paper_count += 1
                }

                adjacent_paper_count = 0;
            }
        }

        // Iterate through unchanged original, count papers
        for (row_pos, row) in rows.iter().enumerate() {
            for (column_pos, column) in row.iter().enumerate() {
                if *column == '@' {
                    original_paper_count += 1;
                }
            }
        }

        // Iterate through copy with removed papers, count papers
        for (row_pos, row) in copy_rows.iter().enumerate() {
            for (column_pos, column) in row.iter().enumerate() {
                if *column == '@' {
                    copy_paper_count += 1;
                }
            }
        }

        println!("copy_paper_count of ROWS: {}", copy_paper_count);

        // Check for equality
        if (original_paper_count == copy_paper_count) || last_loop_paper_count == copy_paper_count {
            break;
        }
    }

    println!("accessable_paper_count: {}", accessable_paper_count);
}

// Handle Corners
fn is_empty_spot(char: &char) -> bool {
    return char == &'.';
}

fn is_top_left(row_pos: usize, column_pos: usize) -> bool {
    return (row_pos == 0) && (column_pos == 0);
}

fn is_top_right(row_pos: usize, column_pos: usize, column_size: usize) -> bool {
    return (row_pos == 0) && (column_pos == column_size - 1);
}

fn is_bottom_left(row_pos: usize, column_pos: usize, row_size: usize) -> bool {
    return (row_pos == row_size - 1) && (column_pos == 0);
}

fn is_bottom_right(row_pos: usize, column_pos: usize, row_size: usize, column_size: usize) -> bool {
    return (row_pos == row_size - 1) && (column_pos == column_size - 1);
}

// Handle Edges
fn is_top_edge(row_pos: usize) -> bool {
    return row_pos == 0;
}

fn is_bottom_edge(row_pos: usize, row_size: usize) -> bool {
    return row_pos == row_size - 1;
}

fn is_right_edge(column_pos: usize, column_size: usize) -> bool {
    return column_pos == column_size - 1;
}

fn is_left_edge(column_pos: usize) -> bool {
    return column_pos == 0;
}
