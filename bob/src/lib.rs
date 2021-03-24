pub fn reply(message: &str) -> &str {
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut question = false;
    let mut all_whitespace = true;

    for c in message.chars().rev() {
        if all_whitespace && c == '?' {
            question = true;
        }
        all_whitespace &= c.is_whitespace();
        has_uppercase |= c.is_uppercase();
        has_lowercase |= c.is_lowercase();
        if has_uppercase && has_lowercase {
            break;
        }
    }
    if all_whitespace {
        return "Fine. Be that way!";
    }
    let yelling = has_uppercase && !has_lowercase;
    match (yelling, question) {
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (true, true) => "Calm down, I know what I'm doing!",
        (false, false) => "Whatever.",
    }
}
