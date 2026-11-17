# nsh-omarchy

[Omarchy](https://omarchy.org)'s look for [NSH (Native Shell)](https://nsh.dev) apps: square corners, one accent,
foreground-tinted state fills, a visible focus border, compact desktop type, and keyboard-first controls. It is a port of
[gpui-omarchy](https://github.com/huacnlee/gpui-omarchy) from GPUI to NSH: `nsh_base` supplies the behavior (focus,
keyboard, popups, accessibility) and this library supplies the presentation.

The colors follow the Omarchy theme the system uses (`~/.local/state/omarchy/current/theme/colors.toml`), switching
live when it changes. Without Omarchy, it draws Tokyo Night (dark) or Flexoki Light.

| Family | Components |
| --- | --- |
| Actions | Button (default, primary, ghost, danger × sm, md, lg), icon button, toggle, toggle group, button group, link |
| Choices | Checkbox, switch, radio group, slider, progress |
| Fields | Text input, textarea, number field, OTP field, select, combobox |
| Navigation | Tabs, menu, context menu, accordion, collapsible, pagination, sidebar |
| Overlays | Dialog, alert dialog, sheet, popover, tooltip, hover card, toast |
| Display | Card, panel, separator, badge, keycap, empty state, avatar, icons, typography, Markdown |
| Data | Table, list, tree, calendar, date picker, color picker, resizable panels |
| Charts | Line, bar, area, pie |

Run the gallery, a page per component, with a theme switch:

```sh
cargo run            # or: nsh run
```

How you use the library depends on the kind of NSH app: [Rust](#in-a-rust-app), [Rust with TSX](#in-a-rust--tsx-app)
or [TSX only](#in-a-tsx-app).

## In a Rust app

Add the crate next to `nsh-core` and `nsh-base` (0.5):

```toml
[dependencies]
nsh-core = "0.5.0"
nsh-base = "0.5.0"
nsh-omarchy = { git = "https://github.com/huacnlee/nsh-omarchy" }
```

Call `nsh_omarchy::init()` once in the root component: it applies the system's Omarchy theme and follows it. Then build
screens from the library's constructors. Each returns the styled `nsh_base` element, so its builder API
(`.on_press`, `.disabled`, `.checked`, `.on_change`, …) is all there. This example is `tests/readme.rs`:

```rust
use nsh_core::prelude::*;
use nsh_omarchy::{self as om, tokens};

#[component]
pub fn App() -> impl IntoElement {
    // Draw with the Omarchy theme the system uses, and follow its changes.
    om::init();
    let saved = signal(0_u32);
    let notify = signal(true);
    view()
        .flex_1()
        .p(px(tokens::SPACE_6))
        .gap(px(tokens::SPACE_4))
        .bg(move || tokens::background())
        .child(
            om::panel("Settings")
                .child(
                    om::switch("Notifications")
                        .checked(move || notify.get())
                        .on_change(move |on: &bool| notify.set(*on)),
                )
                .child(om::button_primary("Save").on_press(move || saved.update(|n| *n += 1))),
        )
        .child(om::body_text(move || format!("Saved {} times", saved.get())))
}
```

- **Colors and sizes come from `nsh_omarchy::tokens`**: `tokens::surface()`, `tokens::accent()`,
  `tokens::hover_fill()`, `tokens::CONTROL_MD`, `tokens::SPACE_4`, `tokens::TEXT_SM`, `tokens::DURATION_FAST`, …
  Read color tokens in a closure (`.bg(move || tokens::surface())`) so they follow a theme change. They are also
  Tailwind classes: `bg-surface`, `text-bright`, `border-omarchy-accent`, `text-text-secondary`.
- **Themes**: `Theme::tokyo_night().apply()` or `Theme::flexoki_light().apply()` pins a palette;
  `nsh_omarchy::follow_system()` goes back to following the system. `active_theme()` is the palette in use.
- **Components are controlled**: you pass the value and write back what `on_change` reports.
- The public API of every family is documented in its file under `src/components/`; the gallery pages in
  `src/gallery/pages/` show each one in use.

## In a Rust + TSX app

A mixed app links the crate and imports its exports in TSX from the bare specifier `nsh-omarchy`. Put the crate (a git
submodule, a checkout, a symlink) in the app and list it in `nsh.json`'s `"rust"`:

```json
{
  "name": "My App",
  "id": "com.example.myapp",
  "version": "1.0.0",
  "build": 1,
  "rust": ["vendor/nsh-omarchy"]
}
```

```tsx
import { mount, signal, Text, View } from "nsh";
import { BarChart, Icon, Markdown, Omarchy, type ThemeInfo } from "nsh-omarchy";

function App() {
  // Rust owns the theme; TSX applies one and hears every change.
  Omarchy.followSystem();
  const theme = signal<ThemeInfo | null>(Omarchy.current());
  Omarchy.onChange((t) => theme.set(t));
  return (
    <View class="flex-1 gap-4 p-6 bg-background">
      <View class="flex-row items-center gap-2">
        <Icon name="sun" label="Theme" />
        <Text class="text-foreground">{() => theme()?.name ?? "…"}</Text>
      </View>
      <BarChart data={[{ label: "Mon", value: 12 }, { label: "Tue", value: 18 }]} accessibilityLabel="Builds" />
      <Markdown source="Read the **manual**." />
    </View>
  );
}

mount(App);
```

| Export | What it is |
| --- | --- |
| `Omarchy` | `followSystem()`, `stopFollowing()`, `applyTheme("tokyo-night" \| "flexoki-light")`, `current(): ThemeInfo \| null`, `onChange(listener): stop` |
| `<LineChart>`, `<AreaChart>`, `<BarChart>`, `<PieChart>` | `data: { label, value }[]` or `series: { name, values }[]`, with `grid`, `xAxis`, `yAxis`, `legend`, `unit`, `stacked`, `curve`, `donut`, … |
| `<Markdown>` | `source`, `selectable`, `onLinkPress(e: { value })` |
| `<Icon>` | `name` (a Lucide name such as `"chevron-down"`), `label`, `size` |
| `<Gallery>` | The whole gallery, as one component |

When the Rust theme changes, the `nsh.json` theme's Tailwind classes follow it (`bg-background`, `text-foreground`,
`bg-surface`, `border-omarchy-accent`), so TSX screens drawn with them match the Rust components. `App.tsx` and
`App.test.tsx` in this repository are a working example; `nsh check`, `nsh build App.tsx` and `nsh test` cover them.

> NSH 0.5.0 from crates.io and the nightly CLI disagree on an `nsh-ios-app` feature, so `nsh check`/`nsh test` of a
> mixed app currently need an NSH CLI that matches the crates, or a local `.cargo/config.toml` that patches `nsh-*` to
> the CLI's own checkout.

## In a TSX app

A TSX-only app links no Rust, so it cannot import the crate's `nsh-omarchy` specifier. It uses the library's TSX
source in `tsx/` instead: the same components rebuilt on `nsh/base` and drawn with the classes of
`themes/omarchy.json`. Put the repository in the app (a git submodule, a checkout, a symlink), name its theme file in
`nsh.json`, and import `tsx/` by path:

```json
{
  "name": "My App",
  "id": "com.example.myapp",
  "theme": "vendor/nsh-omarchy/themes/omarchy.json"
}
```

```json
// tsconfig.json: type check the app and the library's TSX, not the rest of the repository
{
  "extends": "./.nsh/tsconfig.json",
  "include": ["*.ts", "*.tsx", "vendor/nsh-omarchy/tsx"]
}
```

```tsx
import { mount, signal, Text, View } from "nsh";
import { Button, Card, Switch, Toaster, toast } from "./vendor/nsh-omarchy/tsx";

function App() {
  const notify = signal(true);
  const saved = signal(0);
  const save = () => {
    saved.set(saved() + 1);
    toast({ title: "Settings saved" });
  };
  return (
    <View class="flex-1 gap-4 p-6 bg-background">
      <Card title="Settings" footer={<Button label="Save" variant="primary" onPress={save} />}>
        <Switch label="Notifications" checked={notify()} onChange={({ checked }) => notify.set(checked)} />
      </Card>
      <Text class="text-foreground">{() => `Saved ${saved()} times`}</Text>
      <Toaster />
    </View>
  );
}

mount(App);
```

| File | Components |
| --- | --- |
| `button.tsx` | `Button`, `IconButton`, `Toggle`, `ToggleGroup`, `ButtonGroup` |
| `choices.tsx` | `Checkbox`, `Switch`, `RadioGroup`, `Radio`, `Slider`, `Progress` |
| `fields.tsx` | `Input`, `Textarea`, `NumberInput`, `Select`, `Combobox`, `Field` |
| `navigation.tsx` | `Tabs`, `DropdownMenu`, `ContextMenu`, `MenuItem`, `Accordion`, `Pagination`, `Sidebar`, `Link` |
| `overlays.tsx` | `Dialog`, `AlertDialog`, `Sheet`, `Popover`, `Tooltip`, `HoverCard`, `Toaster`, `toast()` |
| `display.tsx` | `Card`, `Panel`, `Separator`, `Badge`, `Keycap`, `Shortcut`, `EmptyState`, `Avatar`, `Heading`, `Title`, `Body`, `Caption`, `Code` |
| `theme.ts` | The class sets the components share (`fill.hover`, `ink.secondary`, `surface.raised`, …) for your own views |

Components are controlled (`checked` with `onChange`, `value` with `onChange`), as in Rust. The theme follows the
system's light or dark appearance (`setColorScheme` from `nsh` pins one); the live Omarchy theme and the charts need the
Rust side, so use a [Rust + TSX app](#in-a-rust--tsx-app) for them. `examples/tsx-gallery` shows every component:

```sh
cd examples/tsx-gallery
nsh run          # the gallery
nsh test         # its headless tests
```

## Developing

```sh
cargo test                   # every component headless: presses, keys, focus, theme following
cargo run                    # the gallery on this machine's desktop backend
nsh run --target web         # or ios, android, macos, linux, windows
```

| Path | Holds |
| --- | --- |
| `src/theme.rs` | The palettes and their projection onto NSH's design tokens |
| `src/system_theme/` | Reading and following the system's Omarchy theme |
| `src/tokens.rs` | The tokens every component draws with |
| `src/components/` | The components, one family per file |
| `src/gallery/` | The gallery app |
| `themes/omarchy.json` | The palettes as an NSH theme file, generated by `cargo test` (`UPDATE_THEME_JSON=1` rewrites it) |
| `DESIGN.md` | The visual language and the rules for code |
