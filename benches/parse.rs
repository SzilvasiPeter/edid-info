#![feature(test)]

extern crate test;

use edid_info::edid::Edid as ZeroCopyEdid;

mod edid_heapallowed;
use edid_heapallowed::Edid as HeapEdid;

const HUGE: &[u8] = include_bytes!("./huge.edid");

#[bench]
fn parse_edid_heap(b: &mut test::Bencher) {
    b.iter(|| {
        let raw = test::black_box(HUGE);
        let out = HeapEdid::parse(raw).expect("parse");
        test::black_box(&out);
    });
}

#[bench]
fn parse_edid_zerocopy(b: &mut test::Bencher) {
    b.iter(|| {
        let raw = test::black_box(HUGE);
        let out = ZeroCopyEdid::parse(raw).expect("parse");
        test::black_box(&out);
    });
}
