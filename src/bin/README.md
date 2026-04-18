# Build

```
cargo build --release --bin check-bytes
```

# Usage

Checks if the minor version is 4 (byte 19), then if the width is not zero (byte 21) but the height (byte 22) is zero:

```
./target/release/check-bytes 'b19 == 4 && b21 != 0 && b22 == 0' ../EDID/
```

# RipGrep on Hex

To match on the raw hex values, use the faster `rg` instead:

```
rg -l "00 00 00 fb 00" ../EDID/
```

# EDID sources

- https://github.com/linuxhw/EDID
- https://github.com/bsdhw/EDID
