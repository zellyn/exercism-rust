use std::char;

fn mines_at(minefield: &[&str], (x, y): (usize, usize)) -> u32 {
    (y < minefield.len() && minefield[y].chars().nth(x) == Some('*')) as u32
}

fn mines_around(minefield: &[&str], (x, y): (usize, usize)) -> u32 {
    let maybe_minus_1 = |n| if n > 0 { n - 1 } else { n };
    (maybe_minus_1(y)..=(y + 1))
        .map(|y| {
            (maybe_minus_1(x)..=(x + 1))
                .map(|x| mines_at(minefield, (x, y)))
                .sum::<u32>()
        })
        .sum()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(y, &s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| match (c, mines_around(minefield, (x, y))) {
                    (' ', n) if n > 0 => char::from_digit(n, 10).unwrap_or('?'),
                    (_, _) => c,
                })
                .collect()
        })
        .collect()
}
