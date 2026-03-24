# Usage

Check invalid week byte:
```
scripts/check_edid_byte.sh 16 'val > 54 && val != 255' ../EDID/
```

Check invalid model year flag on week byte:
```
scripts/check_edid_byte.sh 16 'val == 255' ../EDID/
```

# EDID sources

- https://github.com/linuxhw/EDID
- https://github.com/bsdhw/EDID
