# nsh-omarchy design

An NSH (`nsh-core` / `nsh-base` 0.6) component library, published as an nshm, that brings Omarchy's
look and keyboard-first behavior to NSH apps. It ports the design of
[gpui-omarchy](https://github.com/huacnlee/gpui-omarchy) (read its `docs/design.md` and `src/*.rs`) from
GPUI to NSH: `nsh_base` supplies behavior (focus, keyboard, popups, a11y), this
crate supplies the presentation.

## Visual language (from gpui-omarchy / Omarchy's shell)

- Square corners everywhere (`tokens::RADIUS` = 0). No shadows except popups'
  1px border; no gradients.
- One accent (`tokens::accent()`), used for focus, primary actions, links and a
  menu's cursor text. Never for persistent selection.
- Interaction states are foreground-tinted fills: `normal_fill` (4%),
  `hover_fill` (8%), `selected_fill` (18%), `pressed_fill` (22%).
- Focus is a visible 1px border in `focus_border()` over the hover fill.
  Persistent selection is `selected_fill()` alone (border width 0, no accent
  bar, outline or underline), with `bright()` text and a constant label
  weight. A menu's cursor row is the exception: `hover_fill()` with accent text.
- Row states (hover, focus, pressed, selected) fill the whole row, edge to edge:
  put them on the row element itself, never on an inner content box.
- Primary buttons are transparent with an accent outline and accent text;
  fills mark selected/active state, not priority. Destructive uses `danger()`.
- Compact desktop type: `TEXT_XS` 10 … `TEXT_2XL` 20; controls 24/28/32 tall.
- Disabled: `disabled()` text, no hover/pressed/focus styles.

## Rules for code

- Colors only from `crate::tokens` inside binding closures:
  `.bg(move || tokens::surface())`. Never `rgb(..)` outside `theme.rs`.
- Sizes from `tokens` constants (`px(tokens::CONTROL_MD)`), not magic numbers
  where a token exists.
- The public API is `docs/api.md`, frozen from 0.1.0: names, constructor
  arguments, shorthands and events do not change. New props, components and
  enum variants (every public enum is `#[non_exhaustive]`) may be added.
- Every component is one `#[component]` in `src/ui/`, so Rust and TSX get the
  same thing. Its public constructor lives in `src/<family>.rs` and takes the
  required content (`button("Save")`); everything else is an optional setter
  with a default. Shorthands (`.primary()`, `.small()`) are inherent methods
  on the element, generated in `src/ui/shorthand.rs`.
- Setters take a value, a signal or a closure. Read props in closures where
  they must stay live; read once only what decides structure.
- Values are controlled when given; left out, the component keeps its own.
  Events report the requested value; identity is a string `value`/`id`, never
  an index. Strings in props and events are `SharedString`.
- A control an app may put in a trigger slot draws on a plain view (one
  component deep), so the trigger's press reaches it (nsh-core 0.6).
- Every pressable is an `nsh_base` semantic control (button, menu_item, …) with
  a label; every action works by keyboard with a visible focus.
- Motion: hover/pressed washes use `.transition_colors().duration(tokens::DURATION_FAST)`;
  popups and thumbs use `DURATION_BASE`; sheets `DURATION_SLOW`. Honor reduced motion.

## Layout of the crate

| Path | Holds |
|---|---|
| `src/lib.rs` | Root re-exports, `init()`, `#[native_module("nsh-omarchy")]` |
| `src/<family>.rs` | A family's public constructors, re-exports and data types |
| `src/ui/` | The `#[component]`s and their `register` fns |
| `src/components/` | Drawing helpers shared by the components (crate-private) |
| `src/theme.rs` | `Theme` palettes and their projection onto NSH tokens |
| `src/system_theme/` | Reading/following `~/.local/state/omarchy/current/theme/colors.toml` |
| `src/tokens.rs` | Token functions and constants every component uses |
| `tests/` | Headless tests through the public API |
| `examples/gallery` | The Rust gallery, one page per component |
| `examples/tsx-gallery` | The TSX gallery, installing this crate as an nshm |

## Motion inventory (gallery)

- Sidebar row: hover wash (fast color transition); selection moves the selected fill.
- Page change: content swaps under a keyed region.
- Each component page lists its own state changes in its component docs.
