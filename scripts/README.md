# Usage

Checks if the minor version is 4 (byte 19), then if the width is not zero (byte 21) but the height (byte 22) is zero:

```
./scripts/check_bytes.sh 'bytes[19] == 4 && bytes[21] != 0 && bytes[22] == 0' ../EDID/
```

# EDID sources

- https://github.com/linuxhw/EDID
- https://github.com/bsdhw/EDID
