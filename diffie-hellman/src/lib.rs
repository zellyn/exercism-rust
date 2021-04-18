use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub, a, p)
}

fn pow_mod(base: u64, mut exponent: u64, modulo: u64) -> u64 {
    let mut base = base as u128;
    let modulo = modulo as u128;
    let mut result: u128 = 1;
    base = base % modulo;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % modulo;
        }
        exponent >>= 1;
        base = (base * base) % modulo;
    }
    result as u64
}

/*
// I thought I'd try it with iterators, but it's less clear.
fn pow_mod2(base: u64, exponent: u64, modulo: u64) -> u64 {
    use std::iter::successors;
    let modulo = modulo as u128;
    let filter = successors(
        Some(exponent),
        |&exp| if exp > 1 { Some(exp >> 1) } else { None },
    )
    .map(|exp| exp & 1 > 0);
    let powers = successors(Some(base as u128), |&base| Some((base * base) % modulo));
    filter
        .zip(powers)
        .filter_map(|x| if x.0 { Some(x.1) } else { None })
        .fold(1u128, |result, base| (result * base) % modulo) as u64
}
*/
