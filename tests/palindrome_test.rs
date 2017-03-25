extern crate strutil;

#[test]
fn test_is_palindrome() {
    assert_eq!(true, strutil::is_palindrome(""));
    assert_eq!(true, strutil::is_palindrome("aba"));
    assert_eq!(true, strutil::is_palindrome("racecar"));
    assert_eq!(false, strutil::is_palindrome("abc"));
    assert_eq!(true, strutil::is_palindrome("☺aa☺"));
    assert_eq!(true, strutil::is_palindrome("♥bb♥"));
}