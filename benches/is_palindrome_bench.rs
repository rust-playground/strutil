#![feature(test)]

extern crate test;
extern crate strutil;

use test::Bencher;

#[bench]
fn bench_is_palindrome_multibyte(b: &mut Bencher) {
    b.iter(|| strutil::is_palindrome("♥☺♥☺♥☺♥"));
}

#[bench]
fn bench_is_palindrome_small(b: &mut Bencher) {
    b.iter(|| strutil::is_palindrome("racecar"));
}

#[bench]
fn bench_is_palindrome_mid(b: &mut Bencher) {
    b.iter(|| strutil::is_palindrome("tattarrattat"));
}

#[bench]
fn bench_is_palindrome_large(b: &mut Bencher) {
    b.iter(|| strutil::is_palindrome("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}