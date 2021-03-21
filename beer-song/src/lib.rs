use itertools::Itertools;

pub fn verse(n: u32) -> String {
    let (this_count, next_count) = match n {
        0 => (String::from("no more bottles"), String::from("99 bottles")),
        1 => (String::from("1 bottle"), String::from("no more bottles")),
        2 => (String::from("2 bottles"), String::from("1 bottle")),
        _ => (format!("{} bottles", n), format!("{} bottles", n - 1)),
    };

    let action = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };

    format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        if n == 0 {
            "No more bottles"
        } else {
            &this_count
        },
        this_count,
        action,
        next_count
    )
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).join("\n")
}
