use std::io::{self};

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
    let (mut l, mut r) = (0usize, line.len() - 1);
    let bytes = line.as_bytes();
    while (!is_num(bytes[l]) || !is_num(bytes[r])) && l <= r {
        if !is_num(bytes[l]) {
            l += 1;
        }
        if !is_num(bytes[r]) {
            r -= 1;
        }
    }
    if l > r {
        return None;
    }
    Some((bytes[l] - 0x30) * 10 + bytes[r] - 0x30)
}

fn is_num(v: u8) -> bool {
    0x30 < v && v < 0x3A
}
