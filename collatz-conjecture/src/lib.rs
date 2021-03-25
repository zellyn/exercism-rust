pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    while n > 1 {
        steps += 1;
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    }
    Some(steps)
}
