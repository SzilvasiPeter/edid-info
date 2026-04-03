#![feature(test)]

extern crate test;

use edid_info::edid::Edid as ZeroCopyEdid;

mod edid_heapallowed;
use edid_heapallowed::Edid as HeapEdid;

// TODO: This input has mostly unknown extension blocks. Run other benchmarks with known extensions.
const HUGE: &[u8] = include_bytes!("./huge.edid");

#[bench]
fn parse_edid_heap(b: &mut test::Bencher) {
    b.iter(|| {
        let out = HeapEdid::parse(test::black_box(HUGE)).expect("parse");
        test::black_box(&out);
    });
}

#[bench]
fn parse_edid_zerocopy(b: &mut test::Bencher) {
    b.iter(|| {
        let out = ZeroCopyEdid::parse(test::black_box(HUGE)).expect("parse");
        test::black_box(&out);
    });
}

#[bench]
fn base_edid_heap(b: &mut test::Bencher) {
    let edid = HeapEdid::parse(test::black_box(HUGE)).expect("parse");
    b.iter(|| {
        let base = edid.base();
        test::black_box(base);
    });
}

#[bench]
fn base_edid_zerocopy(b: &mut test::Bencher) {
    let edid = ZeroCopyEdid::parse(test::black_box(HUGE)).expect("parse");
    b.iter(|| {
        let base = edid.base();
        test::black_box(base);
    });
}

#[bench]
fn extensions_edid_heap(b: &mut test::Bencher) {
    let edid = HeapEdid::parse(test::black_box(HUGE)).expect("parse");
    b.iter(|| {
        for ext in edid.extensions() {
            test::black_box(ext);
        }
    });
}

#[bench]
fn extensions_edid_zerocopy(b: &mut test::Bencher) {
    let edid = ZeroCopyEdid::parse(test::black_box(HUGE)).expect("parse");
    b.iter(|| {
        for ext in edid.extensions() {
            test::black_box(ext);
        }
    });
}
