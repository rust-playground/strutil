#![feature(test)]

extern crate test;
extern crate strutil;

use test::Bencher;

#[bench]
fn bench_is_anagram_multibyte(b: &mut Bencher) {
    b.iter(|| strutil::is_anagram("♥☺♥☺♥☺♥", "♥♥♥♥☺☺☺"));
}

#[bench]
fn bench_is_anagram_small(b: &mut Bencher) {
    b.iter(|| strutil::is_anagram("Race car", "Car race"));
}

#[bench]
fn bench_is_anagram_mid(b: &mut Bencher) {
    b.iter(|| strutil::is_anagram("A gentleman", "Elegant man"));
}

#[bench]
fn bench_is_anagram_large(b: &mut Bencher) {
    b.iter(|| strutil::is_anagram("Eleven plus two", "Twelve plus one"));
}