extern crate test;

use std;
use self::test::Bencher;
use lex;
use parse;

#[bench]
fn bench_lex_fizz_buzz(b: &mut Bencher) {
    let src = std::str::from_utf8(include_bytes!("../../res/fizzbuzz.hl2")).unwrap();
    b.iter(|| {
        test::black_box(lex::lex(src, "")).unwrap();
    });
}

#[bench]
fn bench_lex_parse_fizz_buzz(b: &mut Bencher) {
    let src = std::str::from_utf8(include_bytes!("../../res/fizzbuzz.hl2")).unwrap();
    b.iter(|| {
        test::black_box(parse::parse(&lex::lex(src, "").unwrap()[..], src)).unwrap();
    });
}
