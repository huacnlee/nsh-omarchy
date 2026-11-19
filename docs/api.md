# nsh-omarchy API (0.1, stable)

This page is the API contract of `nsh-omarchy`. After 0.1.0 is published nothing on it is renamed,
removed or changes meaning. A later 0.x release only **adds**: new components, new optional props,
new events, new enum variants (every public enum is `#[non_exhaustive]`), new shorthand methods.

Names follow NSH first: where `nsh_base` has the component (`Tabs`, `Tab`, `Menu`, `Toast`,
`AccordionItem`, `ResizablePanelGroup`, `TableHead`, …) this crate uses its name and its event
words (`on_press`, `on_change`, `on_open_change`, `on_select`). Builder sugar follows GPUI Kit:
a constructor takes what the control cannot do without, then `.primary()`, `.small()`, … .

```rust
use nsh_omarchy as om;

om::button("Save").primary().small().on_press(move || save())
om::checkbox("Remember me").checked(move || remember.get()).on_change(move |on: &bool| remember.set(*on))
om::menu()
    .trigger(om::button("File"))
    .child(om::menu_item("New").shortcut("mod-n").on_select(new_file))
```

```tsx
import { Button, Checkbox, Menu, MenuItem } from "nsh-omarchy";

<Button label="Save" variant="primary" size="small" onPress={save} />
<Checkbox label="Remember me" checked={remember()} onChange={({ checked }) => remember.set(checked)} />
<Menu trigger={<Button label="File" />}>
  <MenuItem label="New" shortcut="mod-n" onSelect={newFile} />
</Menu>
```

## 1. Rules every component follows

1. **One implementation.** Each component is one `#[component]`. Rust builds it with its
   constructor; TSX imports it from `"nsh-omarchy"` (installed with `nsh add nsh-omarchy`).
   Same names, props, events and look.
2. **Constructor = required content.** `button(label)`, `radio(value, label)`, `tab(value, label)`.
   In TSX those are the required props. Everything else is optional with a default.
3. **Setters** are named after the prop and take a value, a signal or a closure
   (`.disabled(true)`, `.disabled(move || busy.get())`). Slots take an element
   (`.trigger(om::button("File"))`).
4. **Shorthands** exist only for enum props: variants (`primary()`, `ghost()`, `danger()`, …),
   sizes (`small()`, `medium()`, `large()`) and directions (`horizontal()`, `vertical()`).
   Each is exactly the setter with one value.
5. **Values are controlled when given.** A value prop that is set is owned by the app: the
   component asks for a change with its event and draws the value it is given. Left out, the
   component keeps the value itself. Events report the *requested* value.
6. **Events.** Commands: `on_press` (a click, tap, Return or Space). One value: `value` +
   `on_change`. Several values: `<name>` + `on_<name>_change` (`selected` /
   `on_selected_change`, `sort` / `on_sort_change`). Overlays: `open` + `on_open_change`. Menu
   rows: `on_select`. Text: `on_submit`. In TSX a payload arrives as one object:
   `onChange={({ checked }) => ..}`.
7. **Identity is a string**, never a position: radios, tabs, sidebar items, toggles in a group,
   accordion items, select items, tree items and table rows are named by their `value` / `id`.
8. **Icons are `IconName`** (Lucide): `.icon(IconName::Plus)` in Rust, `icon="plus"` in TSX.
9. **Every component** takes the host props `class` and `testID` in TSX and the `Styled`
   methods in Rust.

### Shared types

| Rust | TSX | Values |
| --- | --- | --- |
| `ControlSize` | `"small" \| "medium" \| "large"` | 24, 28 (default), 32 points tall |
| `ButtonVariant` | `"default" \| "primary" \| "ghost" \| "danger"` | default: control outline |
| `BadgeVariant` | `"default" \| "primary" \| "success" \| "warning" \| "danger"` | |
| `Tone` | `"accent" \| "success" \| "warning" \| "danger"` | progress fill, alert dialog action |
| `IconName` | `"chevron-down" \| …` | every bundled Lucide icon |
| `Axis` | `"horizontal" \| "vertical"` | groups, separators, resizable panels |
| `Placement`, `Align` | from `nsh` | popup side and alignment |
| `Date` | `"2026-11-19"` | a calendar day; `Date::new(y, m, d)`, `Date::today()` |
| `DateRange` | `{ start, end }` | two `Date`s |
| `Hsla` | `string` (any CSS color in, `#rrggbbaa` out) | a color value |

`ControlSize` is not called `Size`: NSH's prelude already has a geometric `Size`.

## 2. Components

Columns: Rust constructor, builders (shorthands in *italics*), events, TSX element. Every
setter is a prop of the same name in TSX (camelCase).

### Actions

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `button(label)` | `variant` *`primary` `ghost` `danger`*, `size` *`small` `medium` `large`*, `icon(IconName)`, `disabled`, `selected`, `loading`, `accessibility_label`, `tooltip(element)`; children after the label | `on_press` | `<Button label>` |
| `icon_button(icon, label)` | as `button` (default variant ghost); `label` is for screen readers | `on_press` | `<IconButton icon label>` |
| `button_group()` | `size`, `disabled`; children: `button`s, joined edge to edge | — | `<ButtonGroup>` |
| `toggle(label)` | `checked`, `icon`, `size`, `disabled`, `value` (inside a group) | `on_change(&bool)` | `<Toggle label>` |
| `toggle_group()` | `value(Vec<String>)`, `single`, `size`, `disabled`; children: `toggle`s with a `value` | `on_change(&Vec<String>)` | `<ToggleGroup>` |
| `link(label, href)` | `open_with(LinkTarget)` *system browser or the app* | `on_press` | `<Link label href>` |

### Choices

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `checkbox(label)` | `checked`, `indeterminate`, `disabled`, `accessibility_label` | `on_change(&bool)` | `<Checkbox label>` |
| `switch(label)` | `checked`, `disabled`, `accessibility_label` | `on_change(&bool)` | `<Switch label>` |
| `radio_group()` | `value(Option<String>)`, `layout(Axis)` *`horizontal` `vertical`*, `disabled`, `accessibility_label`; children: `radio`s | `on_change(&String)` | `<RadioGroup>` |
| `radio(value, label)` | `disabled` | — | `<Radio value label>` |
| `slider()` | `value(SliderValue)` (one value or a range), `min`, `max`, `step`, `disabled`, `show_value`, `accessibility_label` | `on_change(&SliderValue)`, `on_change_end` | `<Slider>` |
| `progress()` | `value(f64)` (0–100), `indeterminate`, `tone` *`success` `warning` `danger`*, `label`, `show_value` | — | `<Progress>` |

### Fields

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `input()` | `value`, `placeholder`, `size`, `disabled`, `readonly`, `invalid`, `masked`, `auto_focus`, `accessibility_label` | `on_change(&String)`, `on_submit(&String)` | `<Input>` |
| `textarea()` | `value`, `placeholder`, `rows`, `disabled`, `readonly`, `invalid`, `accessibility_label` | `on_change(&String)` | `<Textarea>` |
| `number_input()` | `value(Option<f64>)`, `min`, `max`, `step`, `suffix`, `placeholder`, `disabled`, `readonly`, `invalid`, `accessibility_label` | `on_change(&Option<f64>)` | `<NumberInput>` |
| `otp_input(length)` | `value`, `groups`, `masked`, `disabled`, `invalid`, `accessibility_label` | `on_change(&String)`, `on_complete(&String)` | `<OtpInput length>` |
| `select(items)` | `value(Option<String>)`, `placeholder`, `searchable`, `size`, `disabled`, `invalid`, `accessibility_label` | `on_change(&String)` | `<Select items>` |
| `combobox(items)` | `value(Vec<String>)`, `multiple`, `placeholder`, `disabled`, `accessibility_label` | `on_change(&Vec<String>)` | `<Combobox items>` |
| `calendar()` | `value(Option<Date>)`, `range`, `range_value(Option<DateRange>)`, `min`, `max`, `disabled` | `on_change(&Date)`, `on_range_change(&DateRange)` | `<Calendar>` |
| `date_picker()` | as `calendar`, plus `placeholder`, `size`, `accessibility_label`; the trigger shows the value as `Nov 19, 2026` | as `calendar` | `<DatePicker>` |
| `color_picker()` | `value(Hsla)`, `alpha`, `swatches(Vec<Hsla>)`, `disabled`, `accessibility_label` | `on_change(&Hsla)` | `<ColorPicker>` |
| `field(label)` | `description`, `error` (replaces the description, in danger), `required`; child: the control | — | `<Field label>` |

`SelectItem::new(value, label)` with `.group(name)` and `.disabled(bool)`; in TSX
`{ value, label, group?, disabled? }`. Input refs have `focus()`, `blur()`,
`setSelection(start, end?)` and `insertText(text)`.

### Navigation

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `tabs()` | `value(String)`, `variant` *`underline` `segmented`*, `size`, `accessibility_label`; children: `tab`s | `on_change(&String)` | `<Tabs>` |
| `tab(value, label)` | `icon`, `disabled`, `suffix(element)` | — | `<Tab value label>` |
| `sidebar()` | `value(String)`, `header(element)`, `footer(element)`, `accessibility_label`; children: `sidebar_section`s and `sidebar_item`s | `on_change(&String)` | `<Sidebar>` |
| `sidebar_section(title)` | children: `sidebar_item`s | — | `<SidebarSection title>` |
| `sidebar_item(value, label)` | `icon`, `disabled`, `suffix(element)` | — | `<SidebarItem value label>` |
| `accordion()` | `value(Vec<String>)` (the open items), `multiple`, `disabled`; children: `accordion_item`s | `on_change(&Vec<String>)` | `<Accordion>` |
| `accordion_item(value, title)` | `disabled`; children: the content | — | `<AccordionItem value title>` |
| `collapsible(title)` | `open`, `disabled`; children: the content | `on_open_change(&bool)` | `<Collapsible title>` |
| `pagination(page, total_pages)` | `visible_pages`, `disabled`, `accessibility_label` | `on_change(&u32)` | `<Pagination page totalPages>` |
| `menu()` | `trigger(element)`, `open`, `placement`, `align`; children: menu rows | `on_open_change(&bool)` | `<Menu trigger>` |
| `context_menu()` | `menu(rows)`, `disabled`; children: the area it opens over | `on_open_change(&bool)` | `<ContextMenu menu>` |
| `menu_item(label)` | `icon`, `shortcut("mod-n")`, `checked(Option<bool>)`, `danger`, `disabled`, `close_on_select` | `on_select` | `<MenuItem label>` |
| `menu_separator()` | — | — | `<MenuSeparator>` |
| `menu_label(label)` | — | — | `<MenuLabel label>` |
| `title_bar()` | `title`; children after the title (it hides the system's bar and drags the window) | — | `<TitleBar>` |

### Overlays and feedback

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `dialog(title)` | `open`, `description`, `footer(element)`, `close_on_backdrop_press`, `close_on_escape`; children: the body | `on_open_change(&bool)` | `<Dialog title>` |
| `alert_dialog(title, action_label)` | `open`, `description`, `cancel_label`, `tone` *`danger`* | `on_open_change(&bool)`, `on_action` | `<AlertDialog title actionLabel>` |
| `sheet(title)` | `open`, `placement` (default right), `description`, `footer(element)`, `close_label`; children: the body | `on_open_change(&bool)` | `<Sheet title>` |
| `popover()` | `trigger(element)`, `title`, `open`, `placement`, `align`; children: the content | `on_open_change(&bool)` | `<Popover trigger>` |
| `hover_card()` | `trigger(element)`, `placement`; children: the content | `on_open_change(&bool)` | `<HoverCard trigger>` |
| `tooltip(label)` | `shortcut("mod-s")` — given to a control's `tooltip(..)` | — | `<Tooltip label>` |
| `toaster()` | placed once at the root: draws the toasts | — | `<Toaster>` |

```rust
let id = om::toast(om::Toast::new("Settings saved").description("Restart to apply").success()
    .action("Undo", move || undo()));
om::dismiss_toast(&id);
let ok = om::confirm(om::ConfirmOptions::new("Delete 3 files?").action_label("Delete").danger()).await;
```

```tsx
const id = toast({ title: "Settings saved", description: "Restart to apply", kind: "success",
                   action: { label: "Undo", onAction: undo } });
dismissToast(id);
const ok = await confirm({ title: "Delete 3 files?", actionLabel: "Delete", tone: "danger" });
```

`Toast` builders: `description`, `kind(ToastKind)` *`info` `success` `warning` `error`*,
`action(label, fn)`, `timeout(Option<Duration>)` (default `TOAST_TIMEOUT`; `None` stays until
dismissed). Toasting an id that is shown replaces it.

### Data

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `table()` | `accessibility_label`; children: `table_header`, `table_body` | — | `<Table>` |
| `table_header()`, `table_body()` | children: `table_row`s | — | `<TableHeader>`, `<TableBody>` |
| `table_row()` | `selected`, `accessibility_label`; children: cells | `on_press` | `<TableRow>` |
| `table_head(title)` | `width(Option<f32>)`, `numeric` | — | `<TableHead title>` |
| `table_cell(text)` | `width(Option<f32>)`, `numeric` | — | `<TableCell text>` |
| `data_table(columns, rows)` | `selected(Option<String>)`, `sort(Option<TableSort>)`, `accessibility_label`; virtualized, give it a height | `on_selected_change(&String)`, `on_sort_change(&TableSort)`, `on_row_press(&String)` | `<DataTable columns rows>` |
| `list(count, row)` | a virtualized list of `count` rows made by `row(index)`; give it a height | — | NSH's `<List>` |
| `list_item()` | `selected`, `disabled`, `accessibility_label`; children: the content | `on_press` | `<ListItem>` |
| `tree(items)` | `expanded(Vec<String>)`, `selected(Option<String>)`, `accessibility_label`; give it a height | `on_expanded_change(&Vec<String>)`, `on_selected_change(&String)` | `<Tree items>` |
| `scrollbar(target)` | over the scroll view or list `target` names (`ElementRef`) | — | `<Scrollbar target>` |
| `resizable_panel_group()` | `layout(Axis)` *`horizontal` `vertical`*; children: `resizable_panel`s | `on_resize(&Vec<f32>)` | `<ResizablePanelGroup>` |
| `resizable_panel()` | `size(Option<f32>)`, `min_size`, `max_size`; children | — | `<ResizablePanel>` |

- `TableColumn::new(key, title)` with `.width(f32)`, `.numeric()`, `.sortable()`; TSX
  `{ key, title, width?, numeric?, sortable? }`.
- `DataRow::new(id).cell(key, value)`; TSX rows are records with an `id`:
  `{ id: "a", name: "Alpha", size: 12 }`.
- `TableSort { key, direction: SortDirection::Ascending | Descending }`; TSX
  `{ key, direction: "ascending" | "descending" }`. The owner sorts its rows.
- `TreeItem::new(id, label)` with `.icon(..)`, `.disabled(..)`, `.child(TreeItem)`; TSX
  `{ id, label, icon?, disabled?, children? }`.

### Display

| Rust | Builders | Events | TSX |
| --- | --- | --- | --- |
| `card()` | `title`, `description`, `footer(element)`; children: the body | — | `<Card>` |
| `panel(title)` | `variant` *`fill` `outline`*; children | — | `<Panel title>` |
| `separator()` | `layout(Axis)` *`vertical`* | — | `<Separator>` |
| `badge(label)` | `variant` *`primary` `success` `warning` `danger`*, `outline`, `size` | — | `<Badge label>` |
| `keycap(key)` | one key, as Omarchy writes it (`"mod"`, `"shift"`, `"p"`) | — | `<Keycap key>` |
| `shortcut(keys)` | a chord, keycaps joined (`"mod-shift-p"`) | — | `<Shortcut keys>` |
| `avatar(name)` | `src`, `size` | — | `<Avatar name>` |
| `empty_state(title)` | `description`, `icon`; children: actions | — | `<EmptyState title>` |
| `icon(name)` | `size`, `label` (for screen readers; without it the icon is decorative) | — | `<Icon name>` |
| `heading(text)`, `title(text)`, `body(text)`, `caption(text)`, `code(text)` | the type ramp | — | `<Heading>` … `<Code>` with text children |
| `markdown(source)` | `selectable` | `on_link_press(&String)` | `<Markdown source>` |
| `line_chart(data)`, `area_chart(data)`, `bar_chart(data)`, `pie_chart(data)` | `series`, `grid`, `x_axis`, `y_axis`, `legend`, `unit`, `stacked`, `curve`, `donut`, `name`, `accessibility_label` | — | `<LineChart data>` … |

`shortcut_label("mod-s")` returns the text this platform shows (`cmd-s` on macOS, `ctrl-s`
elsewhere); TSX: `Omarchy.shortcutLabel("mod-s")`.

## 3. Theme

```rust
om::init();                       // apply the system's Omarchy theme and follow it
om::Theme::tokyo_night().apply(); // or a built-in palette; stops following
om::stop_following();
let theme = om::active_theme();   // name(), mode()
```

| Rust | TSX (`Omarchy` module) |
| --- | --- |
| `init()` / `follow_system()` | `Omarchy.followSystem()` |
| `stop_following()` | `Omarchy.stopFollowing()` |
| `Theme::tokyo_night().apply()`, `Theme::flexoki_light().apply()` | `Omarchy.applyTheme("tokyo-night" \| "flexoki-light")` |
| `active_theme()` | `Omarchy.current(): ThemeInfo \| null` |
| `on_theme_change(fn) -> Stop` | `Omarchy.onChange(listener): () => void` |
| `Theme::load_system() -> Result<Theme, ThemeLoadError>`, `Theme::from_colors_toml(text, name)` | — |
| `ThemeMode { Light, Dark }`, `theme.name()`, `theme.mode()` | `ThemeInfo { name, mode: "light" \| "dark" }` |

`Theme` has private fields and readers. `tokens` is public (`tokens::accent()`,
`tokens::SPACE_2`, `tokens::DURATION_FAST`); TSX uses the same tokens as classes of
`themes/omarchy.json` (`bg-surface`, `text-secondary`).

## 4. Crate layout

The crate root exports the constructors, element types, builder types, shared types and
`tokens`. The `#[component]`s live in a private module, so the macro's generated no-argument
constructors are not public. The gallery is the `examples/gallery` app, not part of the crate.

## 5. Renamed from the unpublished code

| Before | Now |
| --- | --- |
| `button_primary("Save")`, `button_variant(..)`, `button_with_icon(..)` | `button("Save").primary()`, `.variant(..)`, `.icon(..)` |
| `toggle_sized`, `toggle_with_icon`, `icon_toggle` | `toggle(label).small()`, `.icon(..)` |
| Toggle `pressed` | `checked` |
| `Tabs` (items), `nav_tabs` | `Tabs` + `Tab` children |
| `Sidebar` (sections), `NavSidebar` | `Sidebar` + `SidebarSection` + `SidebarItem` children |
| `ButtonGroup` / `ToggleGroup` (items) | `Button` / `Toggle` children |
| `DropdownMenu`, `menu_dropdown`, `menu_row`, `menu_divider`, `menu_heading` | `Menu`, `menu`, `menu_item`, `menu_separator`, `menu_label` |
| `AccordionSection`, `collapsible_section` | `AccordionItem`, `Collapsible` |
| `resizable(axis)` | `resizable_panel_group().vertical()` |
| `data_column`, `data_column_numeric` | `TableColumn::new(..).numeric()` |
| `list_row` | `list_item` |
| `otp_field`, `text_input`, `number_field`, `select_field`, `combobox_field` | `otp_input`, `input`, `number_input`, `select`, `combobox` |
| `date_picker(label)` | `date_picker()` (formats its own value) |
| `title_bar_title(..)` | `title_bar().title(..)` |
| `Field` `hint` | `description` |
| `Appearance` | `ThemeMode` |
| `heading_text(..)` … | `heading(..)` … |
