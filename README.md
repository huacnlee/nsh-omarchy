# nsh-omarchy

An app written in Rust: its UI is `src/lib.rs`, and `nsh.json` names the root component (`"root": "App"`).

```sh
nsh run                      # this machine's backend; --target ios, android, web, macos, linux or windows
nsh check                    # cargo check of the crate, then type checks App.test.tsx
nsh test                     # cargo test, then App.test.tsx: the root tested from TypeScript, headless; --target to run it on a platform
nsh test nsh.json --trace    # headless: what the root mounts, and the time of each commit
nsh package --target web     # or ios, android, macos, linux, windows
```

`nsh run` and `nsh package` mount the root on every platform, the browser included, without QuickJS: the app is Rust
and native code from start to end (in the browser, with NSH's page script). Rust edits rebuild and restart the app.

```sh
cargo run
```

This starts the root directly on the machine's native desktop backend: GTK 4 on Linux, AppKit on macOS, Win32 on
Windows. Install the backend's development libraries before building.

The `nsh` commands build the app's runners from the NSH crates `Cargo.toml` depends on: crates.io's at the version of
the `nsh` you run (keep the `nsh-*` dependencies at that version when you upgrade `nsh`), or an NSH checkout's by path.

## From TSX: a Rust + TSX app

`#[native_module("nsh-omarchy")]` in `src/lib.rs` exports what only Rust draws well (`src/tsx/`); everyday controls
are the TSX library's (`tsx/`), themed by `themes/omarchy.json`.

| Export | TSX |
| --- | --- |
| `Omarchy` module | `followSystem()`, `stopFollowing()`, `applyTheme("tokyo-night" \| "flexoki-light")`, `current(): ThemeInfo \| null`, `onChange((theme) => …): () => void` |
| Charts | `<LineChart>`, `<AreaChart>`, `<BarChart>`, `<PieChart>`: `data={[{ label, value }]}`, `series={[{ name, values }]}`, `unit`, `decimals`, … |
| `<Markdown>` | `source`, `selectable`, `onLinkPress={(e) => e.value}` |
| `<Icon>` | `name="chevron-down"` (Lucide names), `label`, `size`; tint with `class="text-primary"` |
| `<Gallery>` | Every component, as `nsh run` shows it |

Rust owns the theme: mirror it in a signal from `Omarchy.onChange`; token classes (`bg-primary`) follow it on their
own. `App.tsx` and `App.test.tsx` here exercise each export (`nsh run App.tsx`, `nsh test`); `nsh run` alone keeps
running the Rust root. Another app lists this crate in its `nsh.json` and imports it by its specifier:

```json
{ "name": "My App", "id": "com.example.myapp", "rust": ["vendor/nsh-omarchy"] }
```

```tsx
import { BarChart, Icon, Markdown, Omarchy } from "nsh-omarchy";
```

Add the app's own crate to the list (`"rust": [".", "vendor/nsh-omarchy"]`) when it has one; only listed crates export
to TSX.

NSH-generated files stay in `.nsh/`: runner sources in `.nsh/rust/` and default packages in `.nsh/out/`.
Cargo keeps its normal `target/` directory and respects your existing `CARGO_TARGET_DIR`.

We recommend building with [huacnlee/cargo](https://github.com/huacnlee/cargo), the Cargo NSH is developed with: a
drop-in fork that builds each dependency once for all your projects and Git worktrees and keeps its build directory
under a size limit on its own, with nothing to configure. It builds under `~/.cargo/targets/` and still puts the
binaries in `target/`; it replaces `~/.cargo/bin/cargo`, and its README says how to put rustup's back.

```sh
curl -fsSL https://huacnlee.github.io/cargo/install.sh | sh     # Windows: irm https://huacnlee.github.io/cargo/install.ps1 | iex
```
