pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            ']' | ')' | '}' => if Some(c) != stack.pop() { return false },
            _ => ()
        }
    }
    stack.is_empty()
}
