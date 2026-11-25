//! The data components through the public API, headless: a table row
//! selects, a data table selects and sorts by id, a virtual list keeps its
//! selection by id, a tree folds, resizable panels report their sizes.

use std::time::Duration;

use nsh_core::platform::Os;
use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root, Size};
use nsh_omarchy::{self as om, IconName};
use nsh_testing::{DragOptions, Headless};

const WAIT: Duration = Duration::from_secs(5);

fn start(root: Root) -> Headless {
    let mut config = EngineConfig {
        viewport: Size::new(px(1200.0), px(800.0)),
        ..EngineConfig::new(root)
    };
    // The keyboard reaches rows and trees with Tab, as on a Mac.
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).expect("the engine starts");
    app.first_render().unwrap();
    app
}

// ---- Table --------------------------------------------------------------

#[component]
fn TableHarness() -> impl IntoElement {
    let selected = signal(None::<u32>);
    let rows = [(1, "hyprland"), (2, "waybar"), (3, "nvim")].map(|(id, name)| {
        om::table_row()
            .selected(move || selected.get() == Some(id))
            .on_press(move || selected.set(Some(id)))
            .accessibility_label(name)
            .child(om::table_cell(id.to_string()).width(48.0).numeric(true))
            .child(om::table_cell(name))
    });
    view()
        .child(
            om::table()
                .accessibility_label("Processes")
                .child(
                    om::table_header().child(
                        om::table_row()
                            .child(om::table_head("ID").width(Some(48.0)).numeric(true))
                            .child(om::table_head("Name")),
                    ),
                )
                .child(om::table_body().children(rows)),
        )
        .child(text(move || match selected.get() {
            Some(id) => format!("selected:{id}"),
            None => "selected:none".into(),
        }))
}

#[test]
fn a_table_row_selects_by_press_and_by_keyboard() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<TableHarness>());
    app.wait_for("text:selected:none")?;
    app.wait_for("role:table[Processes]")?;
    app.wait_for("role:columnheader")?;
    app.press("label:waybar")?;
    app.wait_for("text:selected:2")?;
    assert_eq!(app.screen().selected("label:waybar"), Ok(true));
    assert_eq!(app.screen().selected("label:nvim"), Ok(false));
    app.press("label:nvim")?;
    app.wait_for("text:selected:3")?;
    assert_eq!(app.screen().selected("label:waybar"), Ok(false));
    // Body rows are Tab stops; Return presses the focused one. The header
    // row is not.
    app.press_key("tab")?;
    app.press_key("enter")?;
    app.wait_for("text:/selected:[12]/")?;
    Ok(())
}

// ---- Data table ---------------------------------------------------------

fn columns() -> Vec<om::TableColumn> {
    vec![
        om::TableColumn::new("name", "Name").width(160.0).sortable(),
        om::TableColumn::new("size", "Size").width(96.0).numeric().sortable(),
    ]
}

fn files() -> Vec<(&'static str, &'static str, u32)> {
    vec![("a", "alpha", 30), ("b", "bravo", 10), ("c", "charlie", 20)]
}

#[component]
fn DataTableHarness() -> impl IntoElement {
    let sort = signal(None::<om::TableSort>);
    let selected = signal(None::<SharedString>);
    let pressed = signal(SharedString::from("none"));
    let rows = move || {
        let mut files = files();
        if let Some(s) = sort.get() {
            files.sort_by_key(|f| if s.key == "size" { f.2.to_string() } else { f.1.to_string() });
            if s.direction == om::SortDirection::Descending {
                files.reverse();
            }
        }
        files
            .into_iter()
            .map(|(id, name, size)| om::DataRow::new(id).cell("name", name).cell("size", size))
            .collect::<Vec<_>>()
    };
    view()
        .child(
            om::data_table(columns(), rows)
                .accessibility_label("Files")
                .h(px(300.0))
                .w(px(400.0))
                .sort(move || sort.get())
                .selected(move || selected.get())
                .on_sort_change(move |s: &om::TableSort| sort.set(Some(s.clone())))
                .on_selected_change(move |id: &SharedString| selected.set(Some(id.clone())))
                .on_row_press(move |id: &SharedString| pressed.set(id.clone())),
        )
        .child(text(move || {
            let by = sort
                .get()
                .map(|s| format!("{} {:?}", s.key, s.direction))
                .unwrap_or_else(|| "none".into());
            format!(
                "sort:{by} selected:{} pressed:{}",
                selected.get().as_deref().unwrap_or("none"),
                pressed.get()
            )
        }))
}

#[test]
fn a_data_table_selects_and_sorts_by_id() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<DataTableHarness>());
    app.wait_for("text:sort:none selected:none pressed:none")?;
    app.wait_for("text:bravo")?;
    app.press("text:bravo")?;
    app.wait_for("text:sort:none selected:b pressed:b")?;
    // Sorting by size moves bravo first; the selection stays on it.
    app.press("id:sort:size")?;
    app.wait_for("text:sort:size Ascending selected:b pressed:b")?;
    app.press("id:sort:size")?;
    app.wait_for("text:sort:size Descending selected:b pressed:b")?;
    // A third press asks for ascending again, never for no sort.
    app.press("id:sort:size")?;
    app.wait_for("text:sort:size Ascending selected:b pressed:b")?;
    app.press("text:charlie")?;
    app.wait_for("text:sort:size Ascending selected:c pressed:c")?;
    Ok(())
}

#[component]
fn OwnDataTable() -> impl IntoElement {
    let picked = signal(SharedString::from("none"));
    let rows = files()
        .into_iter()
        .map(|(id, name, size)| om::DataRow::new(id).cell("name", name).cell("size", size))
        .collect::<Vec<_>>();
    view()
        .child(
            om::data_table(columns(), rows)
                .h(px(300.0))
                .on_selected_change(move |id: &SharedString| picked.set(id.clone())),
        )
        .child(text(move || format!("picked:{}", picked.get())))
}

#[test]
fn a_data_table_left_alone_keeps_its_own_selection() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<OwnDataTable>());
    app.wait_for("text:alpha")?;
    app.press("text:charlie")?;
    app.wait_for("text:picked:c")?;
    app.press("text:alpha")?;
    app.wait_for("text:picked:a")?;
    Ok(())
}

// ---- List ---------------------------------------------------------------

const LIST_COUNT: usize = 2000;

#[component]
fn ListHarness() -> impl IntoElement {
    let selected = signal(None::<usize>);
    let target = element_ref();
    let id = |at: Position| at.index() + 1;
    let rows = om::list(LIST_COUNT, move |at: Position| {
        om::list_item()
            .selected(move || selected.get() == Some(id(at)))
            .disabled(move || id(at) == 2)
            .on_press(move || selected.set(Some(id(at))))
            .accessibility_label(move || SharedString::from(format!("Item {}", id(at))))
            .child(text(move || format!("Item {}", id(at))))
    })
    .keys(move || (0..LIST_COUNT).map(|i| i + 1))
    .element_ref(&target)
    .test_id("list")
    .size_full();
    view()
        .child(
            view()
                .relative()
                .w(px(420.0))
                .h(px(320.0))
                .child(rows)
                .child(om::scrollbar(target).absolute().top_0().bottom_0().right_0()),
        )
        .child(text(move || match selected.get() {
            Some(id) => format!("selected item {id}"),
            None => "selected none".into(),
        }))
}

#[test]
fn a_virtual_list_scrolls_and_keeps_its_selection_by_id() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<ListHarness>());
    app.wait_for("label:Item 1")?;
    assert!(!app.screen().shows("Item 1500"), "far rows are not made");
    app.press("label:Item 3")?;
    app.wait_for("text:selected item 3")?;
    // A disabled item takes no press.
    assert_eq!(app.screen().enabled("label:Item 2"), Ok(false));
    // 100 rows down (28 points each): row 101 at the top.
    app.scroll("id:list", px(2800.0))?;
    app.wait_for("label:Item 105")?;
    app.wait_for_gone("label:Item 3", WAIT)?;
    app.press("label:Item 105")?;
    app.wait_for("text:selected item 105")?;
    assert_eq!(app.screen().selected("label:Item 105"), Ok(true));
    // A recycled cell shows another row: not selected.
    assert_eq!(app.screen().selected("label:Item 104"), Ok(false));
    app.scroll("id:list", px(-2800.0))?;
    app.wait_for("label:Item 3")?;
    assert_eq!(app.screen().selected("label:Item 3"), Ok(false));
    app.wait_for("text:selected item 105")?;
    Ok(())
}

// ---- Tree ---------------------------------------------------------------

fn tree_items() -> Vec<om::TreeItem> {
    vec![
        om::TreeItem::new("config", "config")
            .icon(IconName::Folder)
            .child(om::TreeItem::new("hypr", "hypr"))
            .child(om::TreeItem::new("waybar", "waybar")),
        om::TreeItem::new("locked", "locked").disabled(true),
        om::TreeItem::new("README.md", "README.md").icon(IconName::File),
    ]
}

#[component]
fn TreeHarness() -> impl IntoElement {
    let expanded = signal(Vec::<SharedString>::new());
    let selected = signal(None::<SharedString>);
    view()
        .child(
            view().w(px(300.0)).h(px(300.0)).child(
                om::tree(tree_items())
                    .accessibility_label("Files")
                    .size_full()
                    .expanded(move || expanded.get())
                    .on_expanded_change(move |next: &Vec<SharedString>| expanded.set(next.clone()))
                    .selected(move || selected.get())
                    .on_selected_change(move |id: &SharedString| selected.set(Some(id.clone()))),
            ),
        )
        .child(text(move || {
            format!(
                "selected:{} expanded:{}",
                selected.get().as_deref().unwrap_or("none"),
                expanded.get().join(",")
            )
        }))
}

#[test]
fn a_tree_folder_expands_and_collapses() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<TreeHarness>());
    app.wait_for("role:treeitem[config]")?;
    assert!(!app.screen().shows("hypr"), "a closed folder hides its items");
    app.press("role:treeitem[config]")?;
    app.wait_for("text:selected:config expanded:config")?;
    app.wait_for("role:treeitem[hypr]")?;
    app.press_key("tab")?;
    app.press_key("down")?;
    app.wait_for("text:selected:hypr expanded:config")?;
    app.press_key("left")?;
    app.wait_for("text:selected:config expanded:config")?;
    app.press_key("left")?;
    app.wait_for("text:selected:config expanded:")?;
    app.wait_for_gone("role:treeitem[hypr]", WAIT)?;
    app.press_key("right")?;
    app.wait_for("role:treeitem[waybar]")?;
    // A disabled item takes no press.
    assert_eq!(app.screen().enabled("role:treeitem[locked]"), Ok(false));
    Ok(())
}

#[component]
fn OwnTree() -> impl IntoElement {
    om::tree(tree_items()).h(px(300.0))
}

#[test]
fn a_tree_left_alone_keeps_its_own_state() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<OwnTree>());
    app.press("role:treeitem[config]")?;
    app.wait_for("role:treeitem[hypr]")?;
    app.press("role:treeitem[hypr]")?;
    app.wait_until("hypr selected", WAIT, |s| s.selected("role:treeitem[hypr]") == Ok(true))?;
    Ok(())
}

// ---- Resizable ----------------------------------------------------------

#[component]
fn ResizableHarness() -> impl IntoElement {
    let sizes = signal(Vec::<f32>::new());
    view()
        .child(
            view().w(px(600.0)).h(px(300.0)).child(
                om::resizable_panel_group()
                    .horizontal()
                    .on_resize(move |next: &Vec<f32>| sizes.set(next.clone()))
                    .child(om::resizable_panel().size(Some(200.0)).child(text("Left")))
                    .child(
                        om::resizable_panel().child(
                            om::resizable_panel_group()
                                .vertical()
                                .child(om::resizable_panel().child(text("Top")))
                                .child(om::resizable_panel().size(100.0).min_size(80.0).child(text("Bottom"))),
                        ),
                    ),
            ),
        )
        .child(text(move || {
            let sizes = sizes.get();
            let parts: Vec<String> = sizes.iter().map(|s| format!("{s:.0}")).collect();
            format!("sizes:{}", parts.join(" "))
        }))
}

#[test]
fn resizable_panels_report_their_sizes_after_a_drag() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<ResizableHarness>());
    app.wait_for("text:Left")?;
    app.wait_for("text:Bottom")?;
    app.wait_for("label:Resize panels vertically")?;
    app.drag(
        "label:Resize panels horizontally",
        Point::new(px(50.0), px(0.0)),
        DragOptions::default(),
    )?;
    app.wait_for("text:sizes:250 350")?;
    Ok(())
}

// ---- TSX declarations ---------------------------------------------------

#[test]
fn the_data_components_are_declared_for_tsx() {
    let ts = nsh_core::element::declarations(&nsh_omarchy::registry(), &[]);
    for name in [
        "Table",
        "TableHeader",
        "TableBody",
        "TableRow",
        "TableHead",
        "TableCell",
        "DataTable",
        "ListItem",
        "Scrollbar",
        "Tree",
        "ResizablePanelGroup",
        "ResizablePanel",
    ] {
        let line = format!("export declare const {name}: ((props: {name}Props) => JSX.Element);");
        assert!(ts.contains(&line), "missing {line:?}");
    }
    for line in [
        "  columns: (TableColumn)[];",
        "  onSortChange?: (event: { sort: TableSort }) => void;",
        "  onSelectedChange?: (event: { selected: string }) => void;",
        "  onRowPress?: (event: { id: string }) => void;",
        "  onExpandedChange?: (event: { expanded: (string)[] }) => void;",
        "  onResize?: (event: { sizes: (number)[] }) => void;",
        "  layout?: Axis;",
    ] {
        assert!(ts.contains(line), "missing {line:?}");
    }
}

// ---- Setter forms -------------------------------------------------------

#[component]
fn SetterForms() -> impl IntoElement {
    view()
        .child(
            om::data_table(columns(), Vec::<om::DataRow>::new())
                .selected("a")
                .selected(None::<SharedString>)
                .selected(Some(SharedString::from("b")))
                .selected(move || Some(SharedString::from("c")))
                .h(px(100.0)),
        )
        .child(
            om::tree(tree_items())
                .selected("config")
                .selected(None::<SharedString>)
                .selected(Some(SharedString::from("hypr")))
                .expanded(None::<Vec<SharedString>>)
                .expanded(vec![SharedString::from("config")])
                .h(px(200.0)),
        )
}

#[test]
fn selected_takes_a_str_an_option_or_a_closure() -> Result<(), String> {
    let mut app = start(nsh_testing::windowed_root::<SetterForms>());
    app.wait_for("role:treeitem[hypr]")?;
    app.wait_until("hypr selected", WAIT, |s| s.selected("role:treeitem[hypr]") == Ok(true))?;
    Ok(())
}
