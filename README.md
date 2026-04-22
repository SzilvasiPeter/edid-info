# edid-info

[![coverage](https://img.shields.io/endpoint?url=https://szilvasipeter.github.io/edid-info/coverage/badge.json)](https://szilvasipeter.github.io/edid-info/coverage/index.html)

Lightweight Rust library for reading EDID data, extracting monitor name, resolution, and refresh rate.

## Features

- Zero-copy parsing
- Extension support: CTA-861
- No external dependencies and `#[no_std]` support
- `#![forbid(unsafe_code)]`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
edid-info = "0.1"
```

## Usage

TODO: add most common usage examples.

TODO: refer to the `examples/` folder.

## API Overview

### Main Types

| Type | Description |
|------|-------------|
| `Edid` | Main entry point for parsing EDID data |
| `BaseEdid` | EDID 1.4 base block (128 bytes) |
| `Extension` | Extension block (CTA) |
| `Cta` | CTA-861 extension block |

### Base Block Accessors

```rust
edid.base().header()      // Manufacturer, product, serial, version
edid.base().basic()       // Display parameters, input type, features
edid.base().chroma()      // Color chromaticity coordinates
edid.base().established() // Established timing flags
edid.base().standard()    // Standard timing identification
edid.base().dtd()         // Detailed timing descriptors
edid.base().footer()      // Extension count, checksum
```

## Implementation References

- **Extended Display Identification Data (EDID) Wikipedia**: https://en.wikipedia.org/wiki/Extended_Display_Identification_Data
- **VESA E-EDID Standard**: https://glenwing.github.io/docs/VESA-EEDID-A2.pdf
- **VESA Display Monitoring Timing (DMT 1.13)**: https://glenwing.github.io/docs/VESA-DMT-1.13.pdf
- **CTA‐861‐G Specification**: https://web.archive.org/web/20171201033424/https://standards.cta.tech/kwspub/published_docs/CTA-861-G_FINAL_revised_2017.pdf
- **Code**:
  - https://git.linuxtv.org/v4l-utils.git/tree/utils/edid-decode/parse-base-block.cpp
  - https://git.linuxtv.org/v4l-utils.git/tree/utils/edid-decode/parse-cta-block.cpp

## License

MIT
