# Working on nsh-omarchy

nsh-omarchy is Omarchy's look for NSH (Native Shell) apps: one Rust crate, an
nshm, that Rust apps call and TSX apps import (`#[native_module("nsh-omarchy")]`).
`nsh-base` owns behavior (focus, keyboard, popups, accessibility); this crate owns
the presentation. It ports gpui-omarchy to NSH.

## Find the relevant guidance

| Work | Read |
|---|---|
| Scope, API design, commits, pull requests, reviews | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Visual language, rules for code, crate layout, motion | [DESIGN.md](DESIGN.md) |
| The public API, frozen from 0.1.0 | [docs/api.md](docs/api.md) |
| The website and its live gallery | [website/README.md](website/README.md) |

## Constraints to keep in mind

- **Every change goes through a pull request. Never push commits to `main`**:
  code, docs, releases, version bumps and CI fixes alike. Create a topic branch,
  push it and open a PR against `main`. A request to "commit and push" or to
  release does not authorize a direct push to `main`.
- Commit subjects and PR titles are `<area>: <What the change does>`
  (CONTRIBUTING.md "Commits and pull requests"). One PR does one thing.
- `docs/api.md` is the contract: names, constructor arguments, shorthands and
  events do not change after 0.1.0. Add, never rename or remove; every public
  enum and options struct is `#[non_exhaustive]`. A PR touching public API lists
  it under `## Public API`.
- Rust and TSX get the same component: one `#[component]` in `src/ui/`, its
  constructor in `src/<family>.rs`, TSX names the Rust names in camelCase.
- Colors and sizes come from `tokens`, read inside closures so they follow a
  theme change. No raw colors outside `src/theme.rs`.
- Behavior that every styled library would need belongs in `nsh-base`, upstream
  in NSH; do not change NSH to fit this crate. Solve it here first.
- Reproduce a defect and fix its cause; no retries, swallowed errors or guards
  that hide it. No unrelated refactors, cleanup or formatting in a change, and do
  not run `cargo fmt` over the whole tree.
- English for code, comments, docs, commit messages and PRs.

## Verification

Run what the change touches, and report what could not be run:

```sh
cargo test                                   # every component headless, through the public API
cargo clippy --all-targets                   # no new warnings in the code you touch
cd examples/tsx-gallery && nsh check && nsh test   # the TSX side, against this checkout
cd website && bun run build && bun run test        # when the website, README or docs/api.md change
```

A visible change is checked on screen as well, in light and dark (the galleries,
`nsh run`), and its PR carries screenshots.
