#!/bin/bash
rg -o '00 ff ff ff ff ff ff 00 [0-9a-f]{2} [0-9a-f]{2}' "$@" | \
    awk -F: '{
        file = $1
        n = split($2, bytes, " ")
        ids = strtonum("0x" bytes[9] bytes[10])
        first = and(rshift(ids, 10), 0x1f)
        second = and(rshift(ids, 5), 0x1f)
        third = and(ids, 0x1f)
        if (first < 1 || first > 26 || second < 1 || second > 26 || third < 1 || third > 26)
            print file
    }'
