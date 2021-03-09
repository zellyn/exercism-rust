#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn invert(c: Comparison) -> Comparison {
    match c {
        Comparison::Equal => Comparison::Equal,
        Comparison::Sublist => Comparison::Superlist,
        Comparison::Superlist => Comparison::Sublist,
        Comparison::Unequal => Comparison::Unequal,
    }
}

// My original solution.
pub fn sublist2<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    let (la, lb) = (a.len(), b.len());
    if la == lb {
        return if a == b {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    }

    if la > lb {
        return invert(sublist(b, a));
    }

    if la == 0 {
        return Comparison::Sublist;
    }

    // a is the needle, b is the text
    for i in 0..(lb - la + 1) {
        if &b[i..i + la] == a {
            return Comparison::Sublist;
        }
    }
    Comparison::Unequal
}

// Me trying to write the top-starred solution without cheating too much.
pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m < n => if b.windows(m).any(|v| v == a) {Sublist} else {Unequal},
        (m, n) if m > n => if a.windows(n).any(|v| v == b) {Superlist} else {Unequal},
        (_, _) => if a == b {Equal} else {Unequal},
    }
}
