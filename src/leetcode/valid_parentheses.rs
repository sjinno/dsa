// Given a string `s` containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// 1. Open brackets must be closed by the same type of brackets.
// 2. Open brackets must be closed in the correct order.
// 3. Every close bracket has a corresponding open bracket of the same type.

fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' if stack.pop() != Some(c) => return false,
            _ => {}
        }
    }
    stack.is_empty()
}

#[test]
fn test_is_valid() {
    let test1 = String::from("()");
    let test2 = String::from("()[]{}");
    let test3 = String::from("(]");
    let test4 = String::from("([])");

    assert!(is_valid(test1));
    assert!(is_valid(test2));
    assert!(!is_valid(test3));
    assert!(is_valid(test4));
}
