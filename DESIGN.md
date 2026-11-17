# nsh-omarchy design

An NSH (`nsh-core` / `nsh-base` 0.5) component library that brings Omarchy's
look and keyboard-first behavior to NSH apps. It ports the design of
[gpui-omarchy](../gpui-omarchy) (read its `docs/design.md` and `src/*.rs`) from
GPUI to NSH: `nsh_base` supplies behavior (focus, keyboard, popups, a11y), this
crate supplies the presentation.

## Visual language (from gpui-omarchy / Omarchy's shell)

- Square corners everywhere (`tokens::RADIUS` = 0). No shadows except popups'
  1px border; no gradients.
- One accent (`tokens::accent()`), used for focus, primary actions and the current item.
- Interaction states are foreground-tinted fills: `normal_fill` (4%),
  `hover_fill` (8%), `selected_fill` (18%), `pressed_fill` (22%).
- Focus is a visible 1px border in `focus_border()`; persistent selection is a
  fill plus a marker (a 2px accent bar or check).
- Primary buttons are transparent with an accent outline and accent text;
  fills mark selected/active state, not priority. Destructive uses `danger()`.
- Compact desktop type: `TEXT_XS` 10 … `TEXT_2XL` 20; controls 24/28/32 tall.
- Disabled: `disabled()` text, no hover/pressed/focus styles.

## Rules for code

- Colors only from `crate::tokens` inside binding closures:
  `.bg(move || tokens::surface())`. Never `rgb(..)` outside `theme.rs`.
- Sizes from `tokens` constants (`px(tokens::CONTROL_MD)`), not magic numbers
  where a token exists.
- Components are controlled: take the value, emit the change, the owner writes.
- Constructors are plain functions returning the styled `nsh_base` element (so
  callers keep its builder API), or `#[component]`s when they need their own
  state/events. Names are snake_case constructors, e.g. `button("Save")`.
- Every pressable is an `nsh_base` semantic control (button, menu_item, …) with
  a label; every action works by keyboard with a visible focus.
- Motion: hover/pressed washes use `.transition_colors().duration(tokens::DURATION_FAST)`;
  popups and thumbs use `DURATION_BASE`; sheets `DURATION_SLOW`. Honor reduced motion.

## Layout of the crate

| Path | Holds |
|---|---|
| `src/theme.rs` | `Theme` palettes and their projection onto NSH tokens |
| `src/system_theme.rs` | Reading/following `~/.local/state/omarchy/current/theme/colors.toml` |
| `src/tokens.rs` | Token functions and constants every component uses |
| `src/components/<family>.rs` | Styled components, one family per file |
| `src/gallery/` | The gallery app (`nsh run`), one page per component |
| `tests/` | Headless tests |

## Motion inventory (gallery)

- Sidebar row: hover wash (fast color transition); selection moves fill + accent bar.
- Page change: content swaps under a keyed region.
- Each component page lists its own state changes in its component docs.
