#!/bin/bash

cond="$1"
shift

rg -l '00 ff ff ff ff ff ff 00' "$@" | while IFS= read -r file; do
    mapfile -t raw < <(tr '\n' ' ' < "$file" | \
            rg -o '00 ff ff ff ff ff ff 00(?: [0-9a-f]{2})+' | \
            head -1 | \
            rg -o '[0-9a-f]{2}' | \
            head -128)

    # Convert hex strings to decimal: "ff" -> "0xff" -> 255
    bytes=()
    for hex in "${raw[@]}"; do
        bytes+=("$(( 0x$hex ))")
    done

    # Transform bytes[N] -> ${bytes[N]} for bash array access
    expr="${cond//bytes\[/\$\{bytes\[}"
    expr="${expr//\]/\]\}}"

    if eval "[[ $expr ]]"; then
        echo "$file"
    fi
done
