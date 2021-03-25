pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return vec![];
    }
    (0..=digits.len() - len)
        .map(|i| String::from(&digits[i..i + len]))
        .collect()
}
