#!/bin/bash
off="${1:-}"
cond="${2:-}"
shift 2 || true
rg -l '00 ff ff ff ff ff ff 00' "$@" | \
    while IFS= read -r file; do
        awk -v off="$off" '
        function ishex(s) { return s ~ /^[0-9a-f]{2}$/ }
        {
            if (!init) {
                split("00 ff ff ff ff ff ff 00", pat, " ")
                init = 1
            }
            for (i = 1; i <= NF; i++) {
                tok = tolower($i)
                if (!ishex(tok)) { continue }
                if (!seen) {
                    if (tok == pat[p + 1]) {
                        p++
                        idx = p - 1
                        if (idx == off) {
                            val = strtonum("0x" tok)
                            if ('"$cond"') { print FILENAME; exit 0 }
                            exit 1
                        }
                        if (p == 8) { seen = 1 }
                    } else {
                        p = (tok == pat[1]) ? 1 : 0
                        if (p == 1 && off == 0) {
                            val = strtonum("0x" tok)
                            if ('"$cond"') { print FILENAME; exit 0 }
                            exit 1
                        }
                    }
                    continue
                }
                idx++
                if (idx == off) {
                    val = strtonum("0x" tok)
                    if ('"$cond"') { print FILENAME; exit 0 }
                    exit 1
                }
            }
        }' "$file"
        if [[ $? -eq 0 ]]; then
            break
        fi
    done
