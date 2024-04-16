
pub fn q1() -> i32 {

    include_bytes!("../input/day2.txt").split(|b| b == &b'\n')
        .enumerate()
        .filter_map(|(game_id, line)| {
            let mut rgb = [0,0,0,0];
            line.splitn(2, |b| b == &b':')
                .nth(1)
                .unwrap()
                .split(|b| b == &b',' || b == &b';')
                .all(|item| {
                    let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                        b"red" => 0usize,
                        b"green" => 1,
                        b"blue" => 2,
                        _ => 3
                    };
                    if i < 4 {rgb[i] = rgb[i].max(atoi::atoi(&item[1..]).unwrap());} else{ rgb[i] = 0;}
                    rgb[i] <= 12 + i as u32
            })
            .then_some(game_id +1)
        })
        .sum::<usize>() as i32
}

pub fn q2() -> i32 {
    include_bytes!("../input/day2.txt").split(|b| b == &b'\n')
        .enumerate()
        .filter_map(|(_, mut line)| {
            let mut rgbmax = [0,0,0];
            line = line.strip_suffix(&[b'\r']).unwrap_or(line);
            line.splitn(2, |b| b == &b':')
                .nth(1)
                .unwrap()
                .split(|b| b == &b',' || b == &b';')
                .all(|item| {
                    let i = match item[1..].splitn(2, |b| *b == b' ').nth(1).unwrap() {
                        b"red" => 0usize,
                        b"green" => 1,
                        b"blue" => 2,
                        _ => unreachable!()
                    };
                    rgbmax[i] = rgbmax[i].max(atoi::atoi(&item[1..]).unwrap());
                    true
            })
            .then_some(rgbmax[0] * rgbmax[1] * rgbmax[2])
        })
    .sum::<usize>() as i32
}
