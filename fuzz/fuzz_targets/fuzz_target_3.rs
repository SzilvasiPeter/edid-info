#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some(block) = data.get(..128).and_then(|b| b.try_into().ok()) else {
        return;
    };
    let ext = edid_info::extensions::Extension::parse(block);
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
});
