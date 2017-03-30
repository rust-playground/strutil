#![feature(test)]

extern crate test;
extern crate strutil;

use test::Bencher;

#[bench]
fn bench_is_anagram_multibyte(b: &mut Bencher) {
    let s1 = "♥☺♥☺♥☺♥";
    let s2 = "♥♥♥♥☺☺☺";
    b.bytes = (s1.bytes().count() + s2.bytes().count()) as u64;
    b.iter(|| strutil::is_anagram(s1, s2));
}

#[bench]
fn bench_is_anagram_small(b: &mut Bencher) {
    let s1 = "Race car";
    let s2 = "Car race";
    b.bytes = (s1.bytes().count() + s2.bytes().count()) as u64;
    b.iter(|| strutil::is_anagram(s1, s2));
}

#[bench]
fn bench_is_anagram_mid(b: &mut Bencher) {
    let s1 = "A gentleman";
    let s2 = "Elegant man";
    b.bytes = (s1.bytes().count() + s2.bytes().count()) as u64;
    b.iter(|| strutil::is_anagram(s1, s2));
}

#[bench]
fn bench_is_anagram_large(b: &mut Bencher) {
    let s1 = "Eleven plus two";
    let s2 = "Twelve plus one";
    b.bytes = (s1.bytes().count() + s2.bytes().count()) as u64;
    b.iter(|| strutil::is_anagram(s1, s2));
}