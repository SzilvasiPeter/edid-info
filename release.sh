#!/usr/bin/env bash
set -euo pipefail

ver=${1:?usage: $0 <version>}
tag="v$ver"

cargo set-version "$ver"
cargo generate-lockfile
git add Cargo.toml Cargo.lock
git commit -m "chore(release): $tag"
git tag "$tag"
git push origin HEAD "$tag"
