# NSH Omarchy website

An Astro site for nsh-omarchy, modeled on the gpui-omarchy website. The homepage embeds the real TSX gallery
(`examples/tsx-gallery`), published with NSH's web backend: the nsh-omarchy crate compiled to WebAssembly, the app's
TSX in the browser's JavaScript engine. The API page renders `docs/api.md` as it is, and the homepage's examples and
component table come from the root `README.md` (whose Rust example `tests/readme.rs` compiles and tests), so the site
cannot drift from them.

## Development

Publish the gallery into `public/gallery/` first. It needs `nsh`, the Rust target `wasm32-unknown-unknown` and a clang
that compiles to WebAssembly (on macOS, Homebrew's `llvm`, which the script finds itself):

```sh
bun install
bun run gallery   # nsh package App.tsx --target web, into public/gallery/
bun run dev       # http://127.0.0.1:4321
```

The gallery follows the site's theme: `examples/tsx-gallery/host.js` (the app's `web.host`) reads the embedding page's
`data-theme` and sends it to the app as a `theme` host event; opened on its own, the gallery keeps its own appearance.

## Validation

```sh
bun run build                      # astro check, then the static site in dist/
bun --bun x playwright install chromium
bun run test                       # against the dev server
PLAYWRIGHT_PRODUCTION=1 bun run test   # against the production build at /nsh-omarchy/
```

`dist/` is a static site with no backend: copy it to any static host. `SITE_URL` and `BASE_PATH` set its public URL
(by default `https://huacnlee.github.io/nsh-omarchy/`, at `/` locally).

## Design

The site follows Omarchy Style's web rules, as gpui-omarchy's site does: the Omarchy display font for the wordmark
line, Geist headings, JetBrains Mono text, square geometry and semantic theme colors, Tokyo Night by default and
Flexoki Light as the light theme. The favicon is a project mark (a stepped N in an open frame), not an official
logo of Omarchy or NSH. The Omarchy display font comes from https://github.com/markcuda/Omarchy-Font and its MIT license
is kept in `public/fonts/OMARCHY-LICENSE`. This is an independent community project, not the official Omarchy website.
