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
        }
        count += (left_num  * 10 )+ right_num;
    }
    count
}

pub fn question2() -> i32 {
    let mut sum = 0;
    for line in fs::read_to_string("input/day1.txt").unwrap().lines() {
        let new_line = line_to_digits(line); 
       
        let mut il = 0;
        let mut ir = 0;
        let mut numl :i32 = 0;
        let mut numr :i32 = 0;
        let mut foundl = false;
        let mut foundr = false;
        
        while ir >= il {

            if new_line.as_bytes()[il].is_ascii_digit() {
                if !foundl {
                    numl = (new_line.as_bytes()[il] - b'0').into();
                    foundl = true;
                }
            }else{
                il += 1;
            }
 
            if new_line.as_bytes()[ir].is_ascii_digit() {
                if !foundr {
                    numr = (new_line.as_bytes()[ir] - b'0').into();
                    foundr = true;
                }
            }else{
                ir -= 1;
            }
            if foundr && foundl {
                break;
            }
        }
        sum += (numl * 10) + numr;
    }
    sum
}


fn line_to_digits(line: &str) -> &str {

    line
}
