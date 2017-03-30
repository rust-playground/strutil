#![feature(test)]

extern crate test;
extern crate strutil;

use test::Bencher;

#[bench]
fn bench_is_palindrome_multibyte(b: &mut Bencher) {
    let s = "♥☺♥☺♥☺♥";
    b.bytes = s.bytes().count() as u64;
    b.iter(|| strutil::is_palindrome(s));
}

#[bench]
fn bench_is_palindrome_small(b: &mut Bencher) {
    let s = "racecar";
    b.bytes = s.bytes().count() as u64;
    b.iter(|| strutil::is_palindrome(s));
}

#[bench]
fn bench_is_palindrome_mid(b: &mut Bencher) {
    let s = "tattarrattat";
    b.bytes = s.bytes().count() as u64;
    b.iter(|| strutil::is_palindrome(s));
}

#[bench]
fn bench_is_palindrome_large(b: &mut Bencher) {
    let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    b.bytes = s.bytes().count() as u64;
    b.iter(|| strutil::is_palindrome(s));
}