mod audio;
mod block;
mod room;
mod speaker;
mod vendor;
mod vic;
mod video;

/// Helper to parse CTA extension from raw bytes in tests.
fn parse_cta(raw: &[u8; 128]) -> edid_info::extensions::cta::Cta<'_> {
    match edid_info::extensions::Extension::parse(raw) {
        edid_info::extensions::Extension::Cta(cta) => cta,
        other @ edid_info::extensions::Extension::Unknown(_) => {
            panic!("expected CTA extension, got {other:?}")
        }
    }
}
