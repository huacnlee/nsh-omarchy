#!/usr/bin/env bash
# Publishes examples/tsx-gallery for the web into public/gallery/, where the
# homepage embeds it. Needs `nsh`, the Rust target wasm32-unknown-unknown and
# a clang that compiles to WebAssembly (on macOS: Homebrew's llvm).
set -euo pipefail
website="$(cd "$(dirname "$0")/.." && pwd)"
gallery="$website/../examples/tsx-gallery"
out="$website/public/gallery"

# On Linux, clang and llvm-ar of the distribution (apt: clang llvm libclang-dev).
if [[ "$(uname)" == "Linux" ]]; then
  export AR_wasm32_unknown_unknown="${AR_wasm32_unknown_unknown:-llvm-ar}"
fi
if [[ "$(uname)" == "Darwin" && -z "${CC_wasm32_unknown_unknown:-}" ]]; then
  llvm="$(brew --prefix llvm 2>/dev/null || true)/bin"
  if [[ -x "$llvm/clang" ]]; then
    export CC_wasm32_unknown_unknown="$llvm/clang" AR_wasm32_unknown_unknown="$llvm/llvm-ar"
  fi
fi

# `nsh package` writes the site into a directory named after the app.
staging="$(mktemp -d)"
trap 'rm -rf "$staging"' EXIT
(cd "$gallery" && nsh package App.tsx --target web --out "$staging")
rm -rf "$out"
mv "$staging"/*/ "$out"
# The page carries the app's own name; keep it out of search results, as the
# homepage that embeds it is the page to find.
sed -i.bak 's#<head>#<head><meta name="robots" content="noindex, follow">#' "$out/index.html" && rm "$out/index.html.bak"
echo "gallery: $out"
