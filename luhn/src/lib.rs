/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned: Vec<u8> = code.bytes().filter(|&b| b >= b'0' && b <= b'9').collect();
    let invalid = code.bytes().any(|b| b != b' ' && (b < b'0' || b > b'9'));
    if cleaned.len() < 2 || invalid {
        return false;
    }
    cleaned.iter().rev().map(|c| (c - b'0') as i32).enumerate().map(|(i, c)| {
        if i % 2 == 0 {
            c
        } else {
            2 * c - 9 * ((c > 4) as i32)
        }
    }).sum::<i32>() % 10 == 0
}
