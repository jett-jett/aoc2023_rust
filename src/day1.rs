use std::fs;

pub fn question1() -> i32 {
    let mut count = 0;
    for line in fs::read_to_string("input/day1.txt").unwrap().lines() {
        let mut left = 0;
        let mut right = line.len() - 1; 
        
        let mut right_num :i32 = 0;
        let mut left_num :i32 = 0;
        let mut left_found = false;
        let mut right_found = false;
        while right >= left {
            if line.as_bytes()[right].is_ascii_digit() {
                right_num = (line.as_bytes()[right] - b'0').into();
                right_found = true;
            }
            else {
                right -= 1;
            }
            if line.as_bytes()[left].is_ascii_digit() {
                left_num = (line.as_bytes()[left] - b'0').into();
                left_found = true;
            }else{
                left += 1;
            }

            if left_found && right_found {
                break;
            }
            print!("{} {} :", left, right);
        }
        count += (left_num  * 10 )+ right_num;
        println!("{}", (left_num * 10 ) + right_num);
    }
    count
}

