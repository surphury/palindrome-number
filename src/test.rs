#[cfg(test)]
use crate::is_palindrome;

#[test]
fn test_1() {
    let result = is_palindrome(121);
    assert_eq!(result, true);
}

#[test]
fn test_2() {
    let result = is_palindrome(-121);
    assert_eq!(result, false);
}

#[test]
fn test_3() {
    let result = is_palindrome(10);
    assert_eq!(result, false);
}
