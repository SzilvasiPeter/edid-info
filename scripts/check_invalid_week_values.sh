#!/bin/bash
while IFS= read -r file; do
    if awk '
        BEGIN {
            split("00 ff ff ff ff ff ff 00", pat, " ")
            found = 0
        }
        function ishex(s) {
            return s ~ /^[0-9a-f]{2}$/
        }
        FNR == 1 {
            idx = 0
            state = 0
            skip = 0
            bad = 0
        }
        {
            if (bad) {
                next
            }
            for (i = 1; i <= NF; i++) {
                tok = tolower($i)
                if (!ishex(tok)) {
                    continue
                }
                if (state == 0) {
                    if (tok == pat[idx + 1]) {
                        idx++
                    } else {
                        idx = (tok == pat[1]) ? 1 : 0
                    }
                    if (idx == 8) {
                        state = 1
                        skip = 8
                        idx = 0
                    }
                } else {
                    if (skip > 0) {
                        skip--
                        continue
                    }
                    val = strtonum("0x" tok)
                    if (val > 54 && val != 255) {
                        print FILENAME
                        bad = 1
                        found = 1
                        break
                    }
                    state = 0
                }
            }
        }
        END {
            exit found ? 0 : 1
        }' "$file"
    then
        break
    fi
done < <(rg -l '00 ff ff ff ff ff ff 00' "$@")
