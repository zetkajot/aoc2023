use std::io::{self};

const NUMS: [&[u8]; 18] = [
    &[0x31],
    &[0x32],
    &[0x33],
    &[0x34],
    &[0x35],
    &[0x36],
    &[0x37],
    &[0x38],
    &[0x39],
    b"one",
    b"two",
    b"six",
    b"four",
    b"five",
    b"nine",
    b"seven",
    b"three",
    b"eight",
];
const NUM_LITERALS_VALUES: [u8; 18] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 6, 4, 5, 9, 7, 3, 8];
const MAX_NUM_LEN: usize = 5;

/// Assuming all chars are ASCII

fn main() -> io::Result<()> {
    let mut sum: u64 = 0;
    let lines = io::stdin().lines();
    for line in lines {
        if let Some(n) = num_from_line(&line?) {
            sum += n as u64;
        }
    }
    print!("{sum}");
    Ok(())
}

fn num_from_line(line: &str) -> Option<u8> {
    let bytes = line.as_bytes();

    let l_num: Option<u8> = {
        let mut v = None;
        'out: for i in 0..bytes.len() {
            for j in 1..=MAX_NUM_LEN {
                let chunk = &bytes[i..(i + j).min(bytes.len())];
                if let Some(num_idx) = to_num_idx(chunk) {
                    v = Some(NUM_LITERALS_VALUES[num_idx]);
                    break 'out;
                }
            }
        }
        v
    };
    let r_num: Option<u8> = {
        let mut v = None;
        'out: for i in (0..bytes.len()).rev() {
            for j in 1..=MAX_NUM_LEN {
                let chunk = &bytes[(i as isize - j as isize + 1).max(0) as usize..=i];
                if let Some(num_idx) = to_num_idx(chunk) {
                    v = Some(NUM_LITERALS_VALUES[num_idx]);
                    break 'out;
                }
            }
        }
        v
    };
    if l_num.is_none() || r_num.is_none() {
        return None;
    }
    Some(l_num.unwrap() * 10 + r_num.unwrap())
}

fn to_num_idx(v: &[u8]) -> Option<usize> {
    let mut result: Option<usize> = None;
    for (idx, num) in NUMS
        .iter()
        .enumerate()
        .take_while(|(_, num)| num.len() <= v.len())
    {
        if &v[0..num.len()] == *num {
            result = Some(idx);
            break;
        }
    }

    result
}
