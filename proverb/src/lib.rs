pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    list.iter()
        .zip(&list[1..])
        .map(|(a, b)| format!("For want of a {} the {} was lost.\n", a, b))
        .collect::<String>()
        + &format!("And all for the want of a {}.", list[0])
}
