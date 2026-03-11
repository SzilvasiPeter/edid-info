# edid-info

[![coverage](https://img.shields.io/endpoint?url=https://szilvasipeter.github.io/edid-info/coverage/badge.json)](https://szilvasipeter.github.io/edid-info/coverage/index.html)

Lightweight Rust library for reading EDID data and extracting monitor name, resolution, and refresh rate from connected displays.

## Features

- Zero-copy parsing of EDID 1.4 base blocks
- Support for CTA-861 and DisplayID 2.0 extensions
- No external dependencies
- `#![forbid(unsafe_code)]`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
edid-info = "0.1"
```

## Usage

### Parse EDID Data

```rust
use edid_info::edid::Edid;

// Load EDID data (e.g., from /sys/class/drm/card0-HDMI-A-1/edid on Linux)
let edid_data = std::fs::read("path/to/edid.bin").unwrap();

// Parse the EDID
let edid = Edid::parse(&edid_data).expect("Failed to parse EDID");

// Access base block information
let maker = edid.base().header().maker();
println!("Manufacturer: {}{}{}", maker[0], maker[1], maker[2]);

let product = edid.base().header().product();
println!("Product code: {}", product);
```

### Extract Monitor Name

```rust
use edid_info::edid::Edid;
use edid_info::edid::dtd::Mode;
use edid_info::edid::descriptor::monitor::DescTag;

let edid = Edid::parse(&edid_data).unwrap();

// Search through DTDs for monitor name
for i in 0..4 {
    if let Some(Mode::Display(desc)) = edid.base().dtd().mode(i) {
        if desc.tag() == DescTag::MonitorName {
            let name = desc.name().unwrap_or("Unknown");
            println!("Monitor name: {}", name);
            break;
        }
    }
}
```

### Get Native Resolution and Refresh Rate

```rust
use edid_info::edid::Edid;
use edid_info::edid::dtd::Mode;

let edid = Edid::parse(&edid_data).unwrap();

// Get the first detailed timing descriptor
if let Some(Mode::Timing(timing)) = edid.base().dtd().mode(0) {
    let h_active = timing.h_active();
    let v_active = timing.v_active();
    let refresh_hz = timing.refresh_rate_hz();
    
    println!("Native resolution: {}x{}@{}Hz", h_active, v_active, refresh_hz / 1000);
}
```

### Parse CTA Extension (HDMI/TV Displays)

```rust
use edid_info::edid::Edid;
use edid_info::edid::Extension;

let edid = Edid::parse(&edid_data).unwrap();

// Iterate through extensions
for ext in edid.extensions() {
    if let Extension::Cta(cta) = ext {
        println!("CTA extension found");
        println!("  Revision: {}", cta.rev());
        println!("  Basic audio: {}", cta.basic_audio());
        println!("  YCbCr 4:4:4: {}", cta.ycbcr_444());
        
        // Get supported video modes
        for svd in cta.data_blocks().filter_map(|b| b.svds()) {
            println!("  VIC {}: {}x{}@{}Hz", svd.vic().code(), svd.width(), svd.height(), svd.vfreq());
        }
    }
}
```

### Verify Checksum

```rust
use edid_info::edid::Edid;

let edid = Edid::parse(&edid_data).unwrap();

// Check base block checksum
assert!(edid.base().checksum_ok());

// Check extension checksums
for ext in edid.extensions() {
    match ext {
        Extension::Cta(cta) => assert!(cta.checksum_ok()),
        Extension::DisplayId(did) => assert!(did.checksum_ok()),
        _ => {}
    }
}
```

## API Overview

### Main Types

| Type | Description |
|------|-------------|
| `Edid` | Main entry point for parsing EDID data |
| `BaseEdid` | EDID 1.4 base block (128 bytes) |
| `Extension` | Extension block (CTA or DisplayID) |
| `Cta` | CTA-861 extension block |
| `DisplayId` | DisplayID 2.0 extension block |

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

## License

MIT
