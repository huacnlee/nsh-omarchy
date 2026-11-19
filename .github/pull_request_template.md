Closes #[issue number]

## Description

Describe in English what this pull request changes and the problem it solves.
Please keep **1 PR to solve 1 problem**, and keep **small improvements small**.

## Screenshot

For visible changes, in both themes. If none, remove this section.

| Theme | Before | After |
| --- | --- | --- |
| Tokyo Night (dark) | [Before] | [After] |
| Flexoki Light | [Before] | [After] |

## Public API

List every public item this pull request adds or changes, in Rust and in TSX, with its signature and one line on
what it is for: constructors, setters, shorthands, events, data types, enum variants, tokens, theme functions. If
none, remove this section.

- `select`: `Select::clearable(bool)` / `<Select clearable />`: shows a button that clears the value.

## Breaking Changes

`docs/api.md` is frozen from 0.1.0, so a pull request normally has none. If it does, describe it with a `diff` block
and why it cannot be an addition instead. If none, remove this section.

```diff
- Old code snippet
+ New code snippet
```

## How to Test

Describe the tests you ran and how to reproduce the change.

## Checklist

- [ ] I have read [CONTRIBUTING.md](../CONTRIBUTING.md) and followed it.
- [ ] Reviewed the changes and confirmed AI generated code (if any) is accurate.
- [ ] Passed `cargo test`, with no new `cargo clippy --all-targets` warnings in the code it touches.
- [ ] Passed `nsh check` and `nsh test` in `examples/tsx-gallery` (for changes to components or their TSX face).
- [ ] Updated `docs/api.md` and the galleries for new or changed public API.
- [ ] Checked visible changes on screen in light and dark, with keyboard and focus.
