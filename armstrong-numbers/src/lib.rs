pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let p = s.len() as u32;
    let sum: u32 = s.bytes().map(|b| ((b - b'0') as u32).pow(p)).sum();
    sum == num
}
