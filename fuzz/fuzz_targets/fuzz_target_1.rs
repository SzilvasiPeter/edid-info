#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(edid) = edid_info::edid::Edid::parse(data) {
        let base = edid.base();
        let _ = base.header();
        let _ = base.basic();
        let _ = base.chroma();
        let _ = base.established_timings();
        let _ = base.standard_timings();
        let _ = base.descriptors();
        let _ = base.footer();

        for ext in edid.extensions() {
            if let edid_info::extensions::Extension::Cta(cta) = ext {
                for block in cta.data_blocks() {
                    let _ = block.svds().count();
                    let _ = block.sads().count();
                    let _ = block.hdmi_vsdb();
                    let _ = block.speaker_alloc();
                    let _ = block.room_config();
                }
                for i in 0..4 {
                    let _ = cta.dtd(i);
                }
            }
        }

        let _ = edid.validate();
    }
});
