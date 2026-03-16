#!/bin/bash
# Find EDID files with extended VICs and potential SVD parsing bugs
# Usage: ./scripts/find_extended_vics.sh /path/to/linuxhw/EDID

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <path-to-linuxhw-EDID-repo>"
    exit 1
fi

EDID_REPO="$1"

if [ ! -d "$EDID_REPO" ]; then
    echo "Error: Directory not found: $EDID_REPO"
    exit 1
fi

echo "Searching for EDIDs with extended VICs..."
echo "=========================================="
echo

# Function to filter out files with extension count mismatches
filter_extension_errors() {
    while read -r file; do
        if [ -n "$file" ] && ! rg -q "extension block\(s\), but found" "$file" 2>/dev/null; then
            echo "$file"
        fi
    done
}

# VIC 65-127 (7-bit extended range)
echo "=== VIC 65-127 (7-bit extended range) ==="
VIC_65_127=$(rg -l "^\s+VIC\s+(6[5-9]|[7-9][0-9]|1[0-1][0-9]|12[0-7]):" "$EDID_REPO" 2>/dev/null | filter_extension_errors | sort -u | head -20)
if [ -n "$VIC_65_127" ]; then echo "$VIC_65_127"; else echo "(none found)"; fi
echo

# VIC 193-219 (8-bit extended range)
echo "=== VIC 193-219 (8-bit extended range) ==="
VIC_193_219=$(rg -l "^\s+VIC\s+(19[3-9]|20[0-9]|21[0-9]):" "$EDID_REPO" 2>/dev/null | filter_extension_errors | sort -u | head -20)
if [ -n "$VIC_193_219" ]; then echo "$VIC_193_219"; else echo "(none found)"; fi
echo

# Copy-ready list
echo "=== Copy-ready file list ==="
{
    echo "$VIC_65_127"
    echo "$VIC_193_219"
} | grep -v "^$" | sort -u | head -20 | while read -r f; do basename "$f"; done
