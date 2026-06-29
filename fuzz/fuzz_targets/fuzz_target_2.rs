#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some(block) = data.get(..128).and_then(|b| b.try_into().ok()) else {
        return;
    };
    let base = edid_info::base::Base::new(block);
    let _ = base.header();
    let _ = base.basic();
    let _ = base.chroma();
    let _ = base.established_timings();
    let _ = base.standard_timings();
    let _ = base.descriptors();
    let _ = base.footer();
    let _ = base.validate();
});
