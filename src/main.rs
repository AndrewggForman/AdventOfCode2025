use std::fs;
fn main() {
    let contents = fs::read_to_string("src/data/d3input.txt")
        .expect("Should have been able to read: src/data/d3input.txt");
}
