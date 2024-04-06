// Завдання 2. Дано рядок символів, що містить дужки різних видів
// (круглі, фігурні, квадратні). Перевірте правильність розставлення в ньому
// круглих дужок, будь-яких дужок.
// Наприклад, [ { ( } ) { ] [ } ( [ ] ) { }.

pub fn check_string(input: &str) -> bool {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_string() {
        assert!(!check_string("[{ ( } ) { ] [ } ( [ ] ) { }"));
        assert!(check_string("[ { ( ) } ]"));
    }

    #[test]
    fn test_check_string_2() {
        assert!(!check_string("[ { ( } ) { ] [ } ( [ ] ) { }"));
        assert!(check_string("[ { ( ) } ]"));
    }

    #[test]
    fn test_check_string_3() {
        assert!(!check_string("[ { ( } ) { ] [ } ( [ ] ) { }"));
        assert!(check_string("[ { ( ) } ]"));
    }
}
