# nsh-omarchy

[Omarchy](https://omarchy.org)'s look for [NSH (Native Shell)](https://nsh.dev) apps: square corners, one accent,
foreground-tinted state fills, a visible focus border, compact desktop type and keyboard-first controls. It is a port
of [gpui-omarchy](https://github.com/huacnlee/gpui-omarchy) to NSH: `nsh-base` supplies the behavior (focus,
keyboard, popups, accessibility), this library the presentation.

The colors follow the Omarchy theme the system uses (`~/.local/state/omarchy/current/theme/colors.toml`) and switch
live when it changes. Without Omarchy, it draws Tokyo Night (dark) or Flexoki Light.

It is an [nshm](https://nsh.dev/docs/nshm.md): one Rust crate that a Rust app calls and a TSX app imports, with the
same components on both sides.

**If this helps you, please [sponsor me](https://github.com/sponsors/huacnlee).**

```sh
nsh add nsh-omarchy
```

| Family | Components |
| --- | --- |
| Actions | Button, IconButton, ButtonGroup, Toggle, ToggleGroup, Link |
| Choices | Checkbox, Switch, RadioGroup + Radio, Slider, Progress |
| Fields | Input, Textarea, NumberInput, OtpInput, Select, Combobox, Calendar, DatePicker, ColorPicker, Field |
| Navigation | Tabs + Tab, Sidebar + SidebarSection + SidebarItem, Accordion + AccordionItem, Collapsible, Pagination, Menu, ContextMenu, MenuItem, MenuSeparator, MenuLabel, TitleBar |
| Overlays | Dialog, AlertDialog, Sheet, Popover, HoverCard, Tooltip, Toaster, `toast()`, `confirm()` |
| Data | Table (+ TableHeader, TableBody, TableRow, TableHead, TableCell), DataTable, ListItem, Tree, Scrollbar, ResizablePanelGroup + ResizablePanel, `list()` (Rust) |
| Display | Card, Panel, Separator, Badge, Keycap, Shortcut, Avatar, EmptyState, Icon, Heading, Title, Body, Caption, Code, Markdown |
| Charts | LineChart, AreaChart, BarChart, PieChart |
| Theme | `Omarchy` (TSX), `Theme`, `follow_system()`, `on_theme_change()` (Rust) |

[docs/api.md](docs/api.md) is the complete API: every component's constructor, props, shorthands and events, in Rust
and TSX.

## In a Rust app

`nsh add nsh-omarchy` adds the crate to the app's `[dependencies]` (or add `nsh-omarchy = "0.2"` yourself). Call
`init()` once in the root component, then build screens from the constructors. A constructor takes the component's
required content; everything else is an optional builder with a default. This example is `tests/readme.rs`:

```rust
use nsh_core::prelude::*;
use nsh_omarchy::{self as om, tokens};

#[component]
pub fn App() -> impl IntoElement {
    // Draw with the system's Omarchy theme, and follow it as it changes.
    om::init();
    let notify = signal(true);
    let saved = signal(0_u32);
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
                .child(om::button("Save").primary().on_press(move || {
                    saved.update(|n| *n += 1);
                    om::toast(om::Toast::new("Settings saved"));
                })),
        )
        .child(om::body(move || format!("Saved {} times", saved.get())))
        .child(om::toaster())
}
```

- **Setters take a value, a signal or a closure.** A closure keeps the prop live: `.checked(move || notify.get())`.
- **Values are controlled when given.** Leave `checked`/`value` out and the component keeps the value itself; give it
  and write back what `on_change` reports.
- **Shorthands** for the common variants: `.primary()`, `.ghost()`, `.danger()`, `.small()`, `.large()`,
  `.vertical()`, `.segmented()`, …
- **Colors and sizes come from `nsh_omarchy::tokens`**: `tokens::surface()`, `tokens::accent()`,
  `tokens::hover_fill()`, `tokens::CONTROL_MD`, `tokens::SPACE_4`, `tokens::TEXT_SM`, `tokens::DURATION_FAST`, …
  Read color tokens in a closure (`.bg(move || tokens::surface())`) so they follow a theme change.
- **Themes**: `Theme::tokyo_night().apply()` or `Theme::flexoki_light().apply()` pins a palette;
  `follow_system()` goes back to following the system; `active_theme()` is the palette in use, and
  `on_theme_change(|theme| ..)` hears every change.

## In a TSX app

`nsh add nsh-omarchy` lists the nshm in `nsh.json`; import its components from `"nsh-omarchy"`. A TSX-only app needs
the Rust toolchain to build its hosts ([nshm](https://nsh.dev/docs/nshm.md)). `examples/tsx-gallery` is an app that
uses every component.

```tsx
import { mount, signal, View } from "nsh";
import { Body, Button, Omarchy, Panel, Switch, Toaster, toast } from "nsh-omarchy";

function App() {
  // Draw with the system's Omarchy theme, and follow it as it changes.
  Omarchy.followSystem();
  const notify = signal(true);
  const saved = signal(0);
  const save = () => {
    saved.set(saved() + 1);
    toast({ title: "Settings saved" });
  };
  return (
    <View class="flex-1 gap-4 p-6 bg-background">
      <Panel title="Settings">
        <Switch label="Notifications" checked={notify()} onChange={({ checked }) => notify.set(checked)} />
        <Button label="Save" variant="primary" onPress={save} />
      </Panel>
      <Body>{`Saved ${saved()} times`}</Body>
      <Toaster />
    </View>
  );
}

mount(() => <App />);
```

Rust's `snake_case` names are `camelCase` in TSX (`on_open_change` is `onOpenChange`), and each event hands one object
(`onChange={({ checked }) => ..}`). `Omarchy` is the theme module: `followSystem()`, `stopFollowing()`,
`applyTheme("tokyo-night" | "flexoki-light")`, `current()`, `onChange(listener)` and `shortcutLabel("mod-s")`.

NSH's standard classes (`bg-background`, `text-foreground`, `border-border`) follow the Omarchy theme as they are. To
draw your own views with Omarchy's extra roles (`bg-surface`, `text-bright`, `text-text-secondary`,
`border-omarchy-accent`), copy [`themes/omarchy.json`](themes/omarchy.json) into the app and name it in `nsh.json`
(`"theme": "omarchy.json"`).

## Developing

```sh
cargo test                       # every component headless: presses, keys, focus, theme following
cd examples/gallery && nsh run   # the Rust gallery, a page per component
cd examples/tsx-gallery
nsh test                         # the TSX gallery's tests, against this checkout
nsh run
cd ../../website                 # the website, with the TSX gallery running in the browser
bun install && bun run gallery && bun run dev
```

| Path | Holds |
| --- | --- |
| `src/<family>.rs` | The public API of a family: constructors, re-exports, data types |
| `src/ui/` | The `#[component]`s, exported to TSX by `#[native_module("nsh-omarchy")]` in `src/lib.rs` |
| `src/components/` | The drawing helpers the components share |
| `src/theme.rs`, `src/system_theme/` | The palettes, and reading and following the system's Omarchy theme |
| `src/tokens.rs` | The tokens every component draws with |
| `themes/omarchy.json` | The palettes as an NSH theme file, generated by `cargo test` (`UPDATE_THEME_JSON=1` rewrites it) |
| `docs/api.md` | The public API |
| `DESIGN.md` | The visual language and the rules for code |
| `website/` | The website: the live gallery (`examples/tsx-gallery` on NSH's web backend) and `docs/api.md` |

## License

MIT
