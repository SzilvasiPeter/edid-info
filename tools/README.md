# Tools

This directory contains helper scripts for development and maintenance.

## `check-bytes`

Checks if specific bytes in EDID files match a given condition. A *super* useful repositories for real world data:

- https://github.com/linuxhw/EDID
- https://github.com/bsdhw/EDID

### Build

```
cargo build --release --bin check-bytes
```

### Usage

Checks if the minor version is 4 (byte 19), then if the width is not zero (byte 21) but the height (byte 22) is zero:

```
./target/release/check-bytes 'b19 == 4 && b21 != 0 && b22 == 0' ../EDID/
```

### RipGrep on Hex

To match on the raw continous hex values, use the faster `rg` instead. For example, get the a EDID files with color point descriptor:

```
rg -l "00 00 00 fb 00" ../EDID/
```

### EDID sources

- https://github.com/linuxhw/EDID
- https://github.com/bsdhw/EDID
