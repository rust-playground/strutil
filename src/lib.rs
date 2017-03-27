//! strutil contains common string functions

use std::cmp::{Ordering, min};
use std::collections::HashMap;

/// Determines if the provided string is a palindrome.
///
/// # Examples
///
/// ```
/// extern crate strutil;
///
/// let s = "racecar";
///
/// assert_eq!(true, strutil::is_palindrome(&s))
/// ```
pub fn is_palindrome(s: &str) -> bool {

    let res = s.chars().cmp(s.chars().rev());

    res == Ordering::Equal
}

/// Determines if the provided string is an anagram of the supplied subject.
///
/// # Rules
///
/// - strings are of same length
/// - an exact character mapping between strings, lowercased and special characters counted.
///
/// # Examples
///
/// ```
/// extern crate strutil;
///
/// let subject = "Eleven plus two";
/// let anagram = "Twelve plus one";
///
/// assert_eq!(true, strutil::is_anagram(&subject, &anagram))
/// ```
pub fn is_anagram(subject: &str, string: &str) -> bool {

    if subject.len() != string.len() || subject == string {
        return false;
    }

    let mut m: HashMap<char, u64> = HashMap::with_capacity(min(subject.len(), 26)); // 26 letters in alphabet

    for c in subject.chars().map(|c| c.to_lowercase().next().unwrap()) {
        let count = m.entry(c).or_insert(0);
        *count += 1;
    }

    let mut remove: bool;

    for c in string.chars().map(|c| c.to_lowercase().next().unwrap()) {

        match m.get_mut(&c) {
            Some(x) => {
                if *x == 1 {
                    remove = true;
                } else {
                    *x -= 1;
                    remove = false;
                }
            }
            None => return false,
        }

        if remove {
            m.remove(&c);
        }
    }

    m.is_empty()
}
