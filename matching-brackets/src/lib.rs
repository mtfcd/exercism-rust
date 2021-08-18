pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            b => {
                let m = if b == '}' {
                    '{'
                } else if b == ']' {
                    '['
                } else if b == ')' {
                    '('
                } else {
                    continue;
                };
                match stack.last() {
                    Some(b) if *b == m => {
                        stack.pop();
                    }
                    _ => return false,
                }
            }
        }
    }

    return stack.is_empty();
}
