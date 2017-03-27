extern crate strutil;

#[test]
fn test_is_anagram() {
    assert_eq!(false, strutil::is_anagram("", ""));
    assert_eq!(false, strutil::is_anagram("abc", "abcd"));
    assert_eq!(true, strutil::is_anagram("Race car", "Car race"));
    assert_eq!(true, strutil::is_anagram("Angel", "glean"));
    assert_eq!(true, strutil::is_anagram("Madam Curie", "Radium came"));
    assert_eq!(true,
               strutil::is_anagram("Eleven plus two", "Twelve plus one"));
    assert_eq!(true, strutil::is_anagram("A gentleman", "Elegant man"));
    assert_eq!(true,
               strutil::is_anagram("♥☺♥☺♥☺♥", "♥♥♥♥☺☺☺"));
}