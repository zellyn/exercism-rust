// See https://exercism.io/tracks/rust/exercises/nth-prime/solutions/fc48d015d75c48b5bba6b9a0d923c963
// for a more impressive, lazy-iterator, solution.
pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3, 5];
    let i = n as usize;
    let mut nn = 5;

    while i + 1 >= primes.len() {
        nn += 2;
        if !primes.iter().any(|x| nn % x == 0) {
            primes.push(nn);
        }
    }
    primes[i]
}
