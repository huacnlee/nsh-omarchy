//! Gallery pages of the `data` group. Each `pub fn` is one page.

use nsh_base::{
    ColorPickerEvent, DataTableColumn, DataTableRows, DataTableSelection, DataTableSort,
    DataTableSortDirection, Date, PickedDate, TreeItem,
};
use nsh_core::geometry::Axis;
use nsh_core::prelude::*;

use crate::components as c;
use crate::tokens;

/// A titled block of a page: a small secondary title over its demo.
fn section(title: &'static str, body: impl IntoElement) -> View {
    view()
        .gap(px(tokens::SPACE_2))
        .child(
            text(title)
                .text_size(px(tokens::TEXT_SM))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::secondary()),
        )
        .child(body)
}

/// A page: its sections stacked.
fn page() -> View {
    view().gap(px(tokens::SPACE_6)).w_full()
}

/// A line of state under a demo.
fn status(line: impl Fn() -> String + 'static) -> Text {
    text(line)
        .text_size(px(tokens::TEXT_SM))
        .font_family(move || tokens::font_mono())
        .text_color(move || tokens::secondary())
}

// ---- Table --------------------------------------------------------------

/// A process the tables show.
#[derive(Clone, PartialEq)]
struct Process {
    pid: u32,
    name: &'static str,
    user: &'static str,
    cpu: f64,
    memory: f64,
}

const NAMES: [&str; 8] = [
    "hyprland",
    "waybar",
    "alacritty",
    "nvim",
    "firefox",
    "mako",
    "pipewire",
    "walker",
];

fn processes(n: usize) -> Vec<Process> {
    (0..n)
        .map(|i| Process {
            pid: 1200 + i as u32 * 7,
            name: NAMES[i % NAMES.len()],
            user: if i % 3 == 0 { "root" } else { "omarchy" },
            cpu: ((i * 37) % 1000) as f64 / 10.0,
            memory: ((i * 113) % 4096) as f64 / 4.0,
        })
        .collect()
}

pub fn table() -> AnyElement {
    let few = processes(5);
    let selected = signal(Some(few[1].pid));

    let rows = few.into_iter().map(move |p| {
        let pid = p.pid;
        c::table_row(move || selected.get() == Some(pid))
            .on_press(move || selected.set(Some(pid)))
            .accessibility_label(SharedString::from(format!("{} {}", p.name, p.pid)))
            .child(c::table_cell_numeric(p.pid.to_string(), Some(64.0)))
            .child(c::table_cell(p.name, None))
            .child(c::table_cell(p.user, Some(96.0)))
            .child(c::table_cell_numeric(format!("{:.1}", p.cpu), Some(72.0)))
    });
    let simple = view()
        .gap(px(tokens::SPACE_2))
        .child(
            c::table("Processes")
                .child(
                    c::table_header().child(
                        c::table_header_row()
                            .child(c::table_head_numeric("PID", Some(64.0)))
                            .child(c::table_head("Name", None))
                            .child(c::table_head("User", Some(96.0)))
                            .child(c::table_head_numeric("CPU %", Some(72.0))),
                    ),
                )
                .child(c::table_body().children(rows)),
        )
        .child(status(move || match selected.get() {
            Some(pid) => format!("selected pid {pid}"),
            None => "selected none".into(),
        }));

    // The big table: 500 rows, sorted by the owner.
    let all = processes(500);
    let sort = signal(None::<DataTableSort>);
    let chosen = signal(None::<u32>);
    let sorted = computed(move || {
        let mut rows = all.clone();
        if let Some(s) = sort.get() {
            rows.sort_by(|a, b| {
                let o = match s.key.as_ref() {
                    "pid" => a.pid.cmp(&b.pid),
                    "name" => a.name.cmp(b.name),
                    "user" => a.user.cmp(b.user),
                    "cpu" => a.cpu.total_cmp(&b.cpu),
                    _ => a.memory.total_cmp(&b.memory),
                };
                match s.direction {
                    DataTableSortDirection::Ascending => o,
                    DataTableSortDirection::Descending => o.reverse(),
                }
            });
        }
        rows
    });
    let data = move || {
        sorted.with(|rows| {
            DataTableRows::new(rows.len())
                .numbers("pid", rows.iter().map(|r| r.pid as f64))
                .texts("name", rows.iter().map(|r| r.name))
                .texts("user", rows.iter().map(|r| r.user))
                .numbers("cpu", rows.iter().map(|r| r.cpu))
                .numbers("memory", rows.iter().map(|r| r.memory))
        })
    };
    let sortable = |mut column: DataTableColumn| {
        column.sortable = true;
        column
    };
    // The selection follows the chosen process through a sort.
    let selection = move || {
        let pid = chosen.get()?;
        let row = sorted.with(|rows| rows.iter().position(|r| r.pid == pid))?;
        let mut selection = DataTableSelection::default();
        selection.row = Some(row as u32);
        Some(selection)
    };
    let big = view()
        .gap(px(tokens::SPACE_2))
        .child(
            c::data_table("All processes")
                .h(px(320.0))
                .columns(vec![
                    sortable(c::data_column_numeric("pid", "PID", 80.0)),
                    sortable(c::data_column("name", "Name", 160.0)),
                    sortable(c::data_column("user", "User", 120.0)),
                    sortable(c::data_column_numeric("cpu", "CPU %", 96.0)),
                    sortable(c::data_column_numeric("memory", "Memory MB", 120.0)),
                ])
                .data(data)
                .sort(move || sort.get())
                .selection(selection)
                .on_sort_change(move |next: &Option<DataTableSort>| sort.set(next.clone()))
                .on_selection_change(move |s: &Option<DataTableSelection>| {
                    let pid = s
                        .as_ref()
                        .and_then(|s| s.row)
                        .and_then(|row| sorted.with(|rows| rows.get(row as usize).map(|r| r.pid)));
                    chosen.set(pid);
                }),
        )
        .child(status(move || {
            let by = sort
                .get()
                .map(|s| format!("{} {:?}", s.key, s.direction))
                .unwrap_or_else(|| "none".into());
            let pid = chosen
                .get()
                .map(|p| p.to_string())
                .unwrap_or_else(|| "none".into());
            format!("sort {by}, selected pid {pid}")
        }));

    page()
        .child(section("Table", simple))
        .child(section("Data table: 500 rows, sortable", big))
        .into_element()
}

// ---- List ---------------------------------------------------------------

/// Rows of the list page.
const LIST_COUNT: usize = 2000;

fn item_id(index: usize) -> u32 {
    index as u32 + 1
}

pub fn list() -> AnyElement {
    // Selection lives in data keyed by the row's id, not in the cells.
    let selected = signal(None::<u32>);
    let target = element_ref();

    let rows = c::list(LIST_COUNT, move |at: Position| {
        c::list_row(move || selected.get() == Some(item_id(at.index())))
            .on_press(move || selected.set(Some(item_id(at.index()))))
            .accessibility_label(move || {
                SharedString::from(format!("Item {}", item_id(at.index())))
            })
            .child(
                text(move || format!("{:04}", item_id(at.index())))
                    .font_family(move || tokens::font_mono())
                    .tabular_nums()
                    .text_color(move || tokens::secondary()),
            )
            .child(
                text(move || format!("Item {}", item_id(at.index())))
                    .flex_1()
                    .number_of_lines(1),
            )
            .child(
                text(move || NAMES[at.index() % NAMES.len()])
                    .text_size(px(tokens::TEXT_SM))
                    .text_color(move || tokens::secondary()),
            )
    })
    .keys(move || (0..LIST_COUNT).map(item_id))
    .element_ref(&target)
    .test_id("list")
    .size_full();

    let framed = view()
        .relative()
        .w(px(420.0))
        .h(px(320.0))
        .border_1()
        .border_color(move || tokens::border())
        .child(rows)
        .child(c::scrollbar(target).absolute().top_0().bottom_0().right_0());

    page()
        .child(section(
            "Virtual list: 2000 rows",
            view()
                .gap(px(tokens::SPACE_2))
                .child(framed)
                .child(status(move || match selected.get() {
                    Some(id) => format!("selected item {id}"),
                    None => "selected none".into(),
                })),
        ))
        .into_element()
}

// ---- Tree ---------------------------------------------------------------

fn tree_items() -> Vec<TreeItem> {
    vec![
        TreeItem::new("config", "config").children(vec![
            TreeItem::new("hypr", "hypr").children(vec![
                TreeItem::new("hyprland.conf", "hyprland.conf"),
                TreeItem::new("bindings.conf", "bindings.conf"),
                TreeItem::new("monitors.conf", "monitors.conf"),
            ]),
            TreeItem::new("waybar", "waybar").children(vec![
                TreeItem::new("config.jsonc", "config.jsonc"),
                TreeItem::new("style.css", "style.css"),
            ]),
            TreeItem::new("alacritty.toml", "alacritty.toml"),
        ]),
        TreeItem::new("themes", "themes").children(vec![
            TreeItem::new("tokyo-night", "tokyo-night"),
            TreeItem::new("catppuccin", "catppuccin"),
            TreeItem::new("gruvbox", "gruvbox").disabled(true),
        ]),
        TreeItem::new("README.md", "README.md"),
    ]
}

pub fn tree() -> AnyElement {
    let expanded = signal(vec![SharedString::from("config")]);
    let selected = signal(Some(SharedString::from("hypr")));
    let demo = view()
        .gap(px(tokens::SPACE_2))
        .child(
            view()
                .w(px(320.0))
                .h(px(300.0))
                .border_1()
                .border_color(move || tokens::border())
                .child(
                    c::tree("Files")
                        .size_full()
                        .items(tree_items())
                        .expanded(move || Some(expanded.get()))
                        .on_expanded_change(move |next: &Vec<SharedString>| {
                            expanded.set(next.clone())
                        })
                        .selected(move || selected.get())
                        .on_selected_change(move |id: &SharedString| {
                            selected.set(Some(id.clone()))
                        }),
                ),
        )
        .child(status(move || {
            format!(
                "selected {}, expanded {}",
                selected.get().as_deref().unwrap_or("none"),
                expanded.get().join(" ")
            )
        }));
    page()
        .child(section("Tree: arrows move, Left and Right fold", demo))
        .into_element()
}

// ---- Calendar -----------------------------------------------------------

pub fn calendar() -> AnyElement {
    let picked = signal(None::<Date>);
    let span = signal(None::<Date>);
    let shown = |d: Option<Date>, what: &str| {
        d.and_then(|d| c::format_date(&d))
            .map(|d| format!("{what} {d}"))
            .unwrap_or_else(|| format!("{what} none"))
    };
    let single = view()
        .gap(px(tokens::SPACE_2))
        .child(c::calendar().on_change(move |d: &Date| picked.set(Some(*d))))
        .child(status(move || shown(picked.get(), "picked")));
    let range = view()
        .gap(px(tokens::SPACE_2))
        .child(
            c::calendar()
                .range(true)
                .on_change(move |d: &Date| span.set(Some(*d))),
        )
        .child(status(move || shown(span.get(), "range")));
    page()
        .child(
            view()
                .flex_row()
                .flex_wrap()
                .gap(px(tokens::SPACE_6))
                .child(section("Calendar", single))
                .child(section("Range", range)),
        )
        .into_element()
}

// ---- Date picker --------------------------------------------------------

pub fn date_picker() -> AnyElement {
    let picked = signal(None::<PickedDate>);
    let label = move || {
        picked
            .get()
            .and_then(|p| p.value.format("%b %-d, %Y"))
            .unwrap_or_else(|| "Choose a date".into())
    };
    let span = signal(None::<PickedDate>);
    let span_label = move || {
        span.get()
            .and_then(|p| p.value.format("%b %-d, %Y"))
            .unwrap_or_else(|| "Choose dates".into())
    };
    page()
        .child(section(
            "Date picker",
            view()
                .gap(px(tokens::SPACE_2))
                .child(
                    c::date_picker(label)
                        .accessibility_label("Due date")
                        .on_change(move |p: &PickedDate| picked.set(Some(p.clone()))),
                )
                .child(status(move || format!("value {}", label()))),
        ))
        .child(section(
            "Range",
            c::date_picker(span_label)
                .accessibility_label("Trip dates")
                .range(true)
                .on_change(move |p: &PickedDate| span.set(Some(p.clone()))),
        ))
        .into_element()
}

// ---- Color picker -------------------------------------------------------

pub fn color_picker() -> AnyElement {
    let value = signal(color(tokens::accent()));
    let hex = signal(SharedString::from("accent"));
    let swatches = (1..8).map(|i| color(tokens::ansi(i))).collect::<Vec<_>>();
    let editor = c::color_picker_panel(swatches)
        .value(move || Some(value.get()))
        .on_change(move |e: &ColorPickerEvent| {
            if let ColorPickerEvent::Change(next, formatted) = e {
                value.set(*next);
                hex.set(formatted.clone());
            }
        });
    page()
        .child(section(
            "Color picker",
            view()
                .gap(px(tokens::SPACE_2))
                .child(editor)
                .child(status(move || format!("color {}", hex.get()))),
        ))
        .into_element()
}

// ---- Resizable ----------------------------------------------------------

fn pane(title: &'static str) -> View {
    view()
        .size_full()
        .p(px(tokens::SPACE_3))
        .gap(px(tokens::SPACE_1))
        .bg(move || tokens::surface())
        .child(
            text(title)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright()),
        )
        .child(
            text("Drag the line beside me.")
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
}

pub fn resizable() -> AnyElement {
    let sizes = signal(Vec::<Points>::new());
    let demo = view()
        .w_full()
        .h(px(320.0))
        .border_1()
        .border_color(move || tokens::border())
        .child(
            c::resizable(Axis::Horizontal)
                .on_resize(move |next: &Vec<Points>| sizes.set(next.clone()))
                .child(c::resizable_panel().size(px(200.0)).child(pane("Sidebar")))
                .child(
                    c::resizable_panel().child(
                        c::resizable(Axis::Vertical)
                            .child(c::resizable_panel().child(pane("Editor")))
                            .child(c::resizable_panel().size(px(110.0)).child(pane("Terminal"))),
                    ),
                )
                .child(c::resizable_panel().size(px(180.0)).child(pane("Outline"))),
        );
    let sizes_line = status(move || {
        let sizes = sizes.get();
        if sizes.is_empty() {
            "sizes as laid out".into()
        } else {
            let parts: Vec<String> = sizes.iter().map(|s| format!("{:.0}", s.0)).collect();
            format!("sizes {}", parts.join(" "))
        }
    });
    page()
        .child(section(
            "Resizable panels",
            view()
                .gap(px(tokens::SPACE_2))
                .child(demo)
                .child(sizes_line),
        ))
        .into_element()
}
