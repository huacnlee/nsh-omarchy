#!/usr/bin/env bash
# Builds the `nsh` CLI from NSH's source at a pinned commit, for the website's
# build (the gallery is published with `nsh package --target web`). Prints the
# directory that holds `nsh`; in GitHub Actions it is also added to PATH.
#
#   NSH_REV     the commit to build (default below: the first with nshm)
#   NSH_REPO    the repository (default: longbridge/nsh)
#   NSH_SOURCE  where the checkout lives (default: ~/.nsh-source)
set -euo pipefail
rev="${NSH_REV:-1b92fd78d851b640b3945b7f1cb18d6d0b8262b1}"
repo="${NSH_REPO:-https://github.com/longbridge/nsh.git}"
source="${NSH_SOURCE:-$HOME/.nsh-source}"

if [[ ! -d "$source/.git" ]]; then
  git init -q "$source"
  git -C "$source" remote add origin "$repo"
fi
if [[ "$(git -C "$source" rev-parse -q --verify HEAD 2>/dev/null || true)" != "$rev" ]]; then
  git -C "$source" fetch -q --depth 1 origin "$rev"
  git -C "$source" checkout -q --detach FETCH_HEAD
fi

# NSH's rust-toolchain.toml lists the iOS and Android targets of its releases;
# the CLI needs none of them. The CLI keeps the checkout's own target dir, as it
# builds the web runtime from the checkout it was built in.
(cd "$source" && env -u CARGO_TARGET_DIR RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}" \
  cargo build --release --locked -p nsh-cli)
bin="$source/target/release"
"$bin/nsh" --version >&2
[[ -n "${GITHUB_PATH:-}" ]] && echo "$bin" >> "$GITHUB_PATH"
echo "$bin"
