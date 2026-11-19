//! The data pages: tables, a virtual list, a tree, resizable panels.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, IconName, tokens};

use crate::registry::Page;

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
        om::table_row()
            .selected(move || selected.get() == Some(pid))
            .on_press(move || selected.set(Some(pid)))
            .accessibility_label(format!("{} {}", p.name, p.pid))
            .child(om::table_cell(p.pid.to_string()).width(64.0).numeric(true))
            .child(om::table_cell(p.name))
            .child(om::table_cell(p.user).width(96.0))
            .child(om::table_cell(format!("{:.1}", p.cpu)).width(72.0).numeric(true))
    });
    let simple = view()
        .gap(px(tokens::SPACE_2))
        .child(
            om::table()
                .accessibility_label("Processes")
                .child(
                    om::table_header().child(
                        om::table_row()
                            .child(om::table_head("PID").width(64.0).numeric(true))
                            .child(om::table_head("Name"))
                            .child(om::table_head("User").width(96.0))
                            .child(om::table_head("CPU %").width(72.0).numeric(true)),
                    ),
                )
                .child(om::table_body().children(rows)),
        )
        .child(status(move || match selected.get() {
            Some(pid) => format!("selected pid {pid}"),
            None => "selected none".into(),
        }));

    // The big table: 500 rows, sorted by the owner.
    let all = processes(500);
    let sort = signal(None::<om::TableSort>);
    let chosen = signal(None::<SharedString>);
    let rows = move || {
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
                    om::SortDirection::Descending => o.reverse(),
                    _ => o,
                }
            });
        }
        rows.iter()
            .map(|p| {
                om::DataRow::new(p.pid.to_string())
                    .cell("pid", p.pid)
                    .cell("name", p.name)
                    .cell("user", p.user)
                    .cell("cpu", format!("{:.1}", p.cpu))
                    .cell("memory", format!("{:.0}", p.memory))
            })
            .collect::<Vec<_>>()
    };
    let columns = vec![
        om::TableColumn::new("pid", "PID").width(80.0).numeric().sortable(),
        om::TableColumn::new("name", "Name").width(160.0).sortable(),
        om::TableColumn::new("user", "User").width(120.0).sortable(),
        om::TableColumn::new("cpu", "CPU %").width(96.0).numeric().sortable(),
        om::TableColumn::new("memory", "Memory MB").width(120.0).numeric().sortable(),
    ];
    let big = view()
        .gap(px(tokens::SPACE_2))
        .child(
            om::data_table(columns, rows)
                .accessibility_label("All processes")
                .h(px(320.0))
                .sort(move || sort.get())
                .selected(move || chosen.get())
                .on_sort_change(move |next: &om::TableSort| sort.set(Some(next.clone())))
                .on_selected_change(move |id: &SharedString| chosen.set(Some(id.clone()))),
        )
        .child(status(move || {
            let by = sort
                .get()
                .map(|s| format!("{} {:?}", s.key, s.direction))
                .unwrap_or_else(|| "none".into());
            let pid = chosen.get().unwrap_or_else(|| "none".into());
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

    let rows = om::list(LIST_COUNT, move |at: Position| {
        om::list_item()
            .selected(move || selected.get() == Some(item_id(at.index())))
            .on_press(move || selected.set(Some(item_id(at.index()))))
            .accessibility_label(move || SharedString::from(format!("Item {}", item_id(at.index()))))
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
    .test_id("items")
    .size_full();

    let framed = view()
        .relative()
        .w(px(420.0))
        .h(px(320.0))
        .border_1()
        .border_color(move || tokens::border())
        .child(rows)
        .child(om::scrollbar(target).absolute().top_0().bottom_0().right_0());

    page()
        .child(section(
            "Virtual list: 2000 rows, with a scrollbar",
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

fn file(name: &'static str) -> om::TreeItem {
    om::TreeItem::new(name, name).icon(IconName::File)
}

fn folder(name: &'static str) -> om::TreeItem {
    om::TreeItem::new(name, name).icon(IconName::Folder)
}

fn tree_items() -> Vec<om::TreeItem> {
    vec![
        folder("config")
            .child(folder("hypr").children([
                file("hyprland.conf"),
                file("bindings.conf"),
                file("monitors.conf"),
            ]))
            .child(folder("waybar").children([file("config.jsonc"), file("style.css")]))
            .child(file("alacritty.toml")),
        folder("themes").children([
            folder("tokyo-night"),
            folder("catppuccin"),
            folder("gruvbox").disabled(true),
        ]),
        file("README.md"),
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
                    om::tree(tree_items())
                        .accessibility_label("Files")
                        .size_full()
                        .expanded(move || expanded.get())
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
    let sizes = signal(Vec::<f32>::new());
    let demo = view()
        .w_full()
        .h(px(320.0))
        .border_1()
        .border_color(move || tokens::border())
        .child(
            om::resizable_panel_group()
                .on_resize(move |next: &Vec<f32>| sizes.set(next.clone()))
                .child(om::resizable_panel().size(200.0).child(pane("Sidebar")))
                .child(
                    om::resizable_panel().child(
                        om::resizable_panel_group()
                            .vertical()
                            .child(om::resizable_panel().child(pane("Editor")))
                            .child(om::resizable_panel().size(110.0).child(pane("Terminal"))),
                    ),
                )
                .child(om::resizable_panel().size(180.0).child(pane("Outline"))),
        );
    let sizes_line = status(move || {
        let sizes = sizes.get();
        if sizes.is_empty() {
            "sizes as laid out".into()
        } else {
            let parts: Vec<String> = sizes.iter().map(|s| format!("{s:.0}")).collect();
            format!("sizes {}", parts.join(" "))
        }
    });
    page()
        .child(section(
            "Resizable panels",
            view().gap(px(tokens::SPACE_2)).child(demo).child(sizes_line),
        ))
        .into_element()
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Data", id: "table", title: "Table", make: table },
    Page { group: "Data", id: "list", title: "List", make: list },
    Page { group: "Data", id: "tree", title: "Tree", make: tree },
    Page { group: "Data", id: "resizable", title: "Resizable", make: resizable },
];
