pub fn question1() -> i32 {
    include_bytes!("../input/day1.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            ((line.iter().find(|b| b.is_ascii_digit()).unwrap() - b'0') * 10
             + line.iter().rev().find(|b| b.is_ascii_digit()).unwrap() -b'0') as usize 
        })
        .sum::<usize>() as i32
        
}

const NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn question2() -> i32 {
    include_bytes!("../input/day1.txt")
            .split(|b| b == &b'\n')
            .map(|line| {
                (0..line.len()).find_map(|i| num(line, i)).unwrap() * 10
                    + (0..line.len()).rev().find_map(|i| num(line, i)).unwrap()
            })
            .sum::<usize>() as i32
}

#[inline(always)]
fn num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some( if line[i] >= b'0' {(line[i] - b'0') as usize} else { line[i] as usize })
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[i..].starts_with(name))
            .map(|(num, _)| num + 1))
}



