use std::fs;

pub fn question1() -> i32 {
    let mut count = 0;
    for line in fs::read_to_string("input/day1.txt").unwrap().lines() {
        let mut left = 0;
        let mut right = line.len();
        count += 1; 
    }
    count
}
