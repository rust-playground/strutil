use std::cmp::Ordering;

pub fn is_palindrome(s: &str) -> bool {

    let res = s.chars().cmp(s.chars().rev());

    res == Ordering::Equal
}

// pub fn is_palindrome(s: &str) -> bool {

//     let half = s.chars().count() / 2;

//     return s.chars().take(half).eq(s.chars().rev().take(half));
// }

// use std::string::ToString;

// pub fn is_palindrome<T: ToString>(val: T) -> bool {

//     let s = val.to_string();
//     let half = s.chars().count() / 2;

//     s.chars().take(half).eq(s.chars().rev().take(half))
// }

// use std::fmt::Display;

// pub fn is_palindrome<T: Display>(val: T) -> bool {

//     let s = format!("{}", val);
//     let half = s.chars().count() / 2;

//     s.chars().take(half).eq(s.chars().rev().take(half))
// }