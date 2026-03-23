#!/bin/bash
rg -l '00 ff ff ff ff ff ff 00' "$@" | \
    while IFS= read -r file; do
        awk '
        BEGIN {
            split("00 ff ff ff ff ff ff 00", pat, " ")
        }
        function ishex(s) {
            return s ~ /^[0-9a-f]{2}$/
        }
        FNR == 1 {
            idx = 0
            state = 0
            skip = 0
            found = 0
        }
        {
            if (found) {
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
                    if (tok == "ff") {
                        print FILENAME
                        found = 1
                        break
                    }
                    state = 0
                }
            }
        }' "$file"
    done
