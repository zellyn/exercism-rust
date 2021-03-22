// My first solution computed primes, and was very slow. After looking at
// community solutions, I re-wrote mine.
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = vec![];

    let mut n = n;

    for candidate in 2.. {
        while n % candidate == 0 {
            factors.push(candidate);
            n /= candidate;
        }
        if n == 1 { break };
    }

    factors
}
