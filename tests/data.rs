//! The `data` group's components, headless: a table row selects, a tree
//! folder expands, a calendar day is picked, a date picker opens and picks,
//! a virtual list scrolls and keeps its selection by id. Today is
//! 2026-10-28 (the engine's manual clock).

use std::time::{Duration, SystemTime};

use nsh_base::{Date, PickedDate, TreeItem};
use nsh_core::platform::Os;
use nsh_core::prelude::*;
use nsh_core::time::Clock;
use nsh_engine::{EngineConfig, Root, Size};
use nsh_omarchy as omarchy;
use nsh_testing::Headless;

const WAIT: Duration = Duration::from_secs(5);

fn start(root: Root) -> Headless {
    let mut config = EngineConfig {
        viewport: Size::new(px(1200.0), px(800.0)),
        ..EngineConfig::new(root)
    };
    config.environment.os = Os::MacOs;
    // Noon UTC, 2026-10-28.
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH + Duration::from_secs(1_793_188_800));
    let mut app = Headless::start(config).expect("the engine starts");
    app.first_render().unwrap();
    app
}

// ---- Table --------------------------------------------------------------

#[component]
fn TableHarness() -> impl IntoElement {
    let selected = signal(None::<u32>);
    let rows = [(1, "hyprland"), (2, "waybar"), (3, "nvim")].map(|(id, name)| {
        omarchy::table_row(move || selected.get() == Some(id))
            .on_press(move || selected.set(Some(id)))
            .accessibility_label(name)
            .child(omarchy::table_cell_numeric(id.to_string(), Some(48.0)))
            .child(omarchy::table_cell(name, None))
    });
    view()
        .child(
            omarchy::table("Processes")
                .child(
                    omarchy::table_header().child(
                        omarchy::table_header_row()
                            .child(omarchy::table_head_numeric("ID", Some(48.0)))
                            .child(omarchy::table_head("Name", None)),
                    ),
                )
                .child(omarchy::table_body().children(rows)),
        )
        .child(text(move || match selected.get() {
            Some(id) => format!("selected:{id}"),
            None => "selected:none".into(),
        }))
}

#[test]
fn a_table_row_selects_by_press_and_by_keyboard() -> Result<(), String> {
    let mut app = start(Root::of::<TableHarness>());
    app.wait_for("text:selected:none")?;
    app.press("label:waybar")?;
    app.wait_for("text:selected:2")?;
    assert_eq!(app.screen().selected("label:waybar"), Ok(true));
    assert_eq!(app.screen().selected("label:nvim"), Ok(false));
    app.press("label:nvim")?;
    app.wait_for("text:selected:3")?;
    assert_eq!(app.screen().selected("label:waybar"), Ok(false));
    // Rows are Tab stops; Return presses the focused one.
    app.press_key("tab")?;
    app.press_key("enter")?;
    app.wait_for("text:/selected:[12]/")?;
    Ok(())
}

// ---- Tree ---------------------------------------------------------------

#[component]
fn TreeHarness() -> impl IntoElement {
    let expanded = signal(Vec::<SharedString>::new());
    let selected = signal(None::<SharedString>);
    view()
        .child(
            view().w(px(300.0)).h(px(300.0)).child(
                omarchy::tree("Files")
                    .size_full()
                    .items(vec![
                        TreeItem::new("config", "config").children(vec![
                            TreeItem::new("hypr", "hypr"),
                            TreeItem::new("waybar", "waybar"),
                        ]),
                        TreeItem::new("README.md", "README.md"),
                    ])
                    .expanded(move || Some(expanded.get()))
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
    let mut app = start(Root::of::<TreeHarness>());
    app.wait_for("role:treeitem[config]")?;
    assert!(
        !app.screen().shows("hypr"),
        "a closed folder hides its items"
    );
    // A press on a folder selects and toggles it.
    app.press("role:treeitem[config]")?;
    app.wait_for("text:selected:config expanded:config")?;
    app.wait_for("role:treeitem[hypr]")?;
    // The keys, once Tab gives the tree the keyboard (on the selected
    // row): Down to the child, Left back to the parent, Left folds.
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
    Ok(())
}

// ---- Calendar -----------------------------------------------------------

#[component]
fn CalendarHarness() -> impl IntoElement {
    let picked = signal(None::<Date>);
    view()
        .child(omarchy::calendar().on_change(move |d: &Date| picked.set(Some(*d))))
        .child(text(move || {
            let shown = picked.get().and_then(|d| omarchy::format_date(&d));
            format!("picked:{}", shown.as_deref().unwrap_or("none"))
        }))
}

#[test]
fn a_calendar_day_is_picked_and_selected() -> Result<(), String> {
    let mut app = start(Root::of::<CalendarHarness>());
    app.wait_for("text:October")?;
    app.wait_for("text:picked:none")?;
    app.press("label:2026-10-14")?;
    app.wait_for("text:picked:Oct 14, 2026")?;
    app.wait_until("the day selected", WAIT, |s| {
        s.selected("label:2026-10-14") == Ok(true)
    })?;
    // The header turns the month; a day of the next one is picked.
    app.press("label:Next month")?;
    app.wait_for("text:November")?;
    app.press("label:2026-11-03")?;
    app.wait_for("text:picked:Nov 3, 2026")?;
    Ok(())
}

// ---- Date picker --------------------------------------------------------

#[component]
fn DatePickerHarness() -> impl IntoElement {
    let picked = signal(None::<PickedDate>);
    let label = move || {
        picked
            .get()
            .and_then(|p| p.value.format("%b %-d, %Y"))
            .unwrap_or_else(|| "Choose a date".into())
    };
    view()
        .p(px(20.0))
        .child(
            omarchy::date_picker(label)
                .accessibility_label("Due date")
                .on_change(move |p: &PickedDate| picked.set(Some(p.clone()))),
        )
        .child(text(move || format!("value:{}", label())))
}

#[test]
fn a_date_picker_opens_picks_and_closes() -> Result<(), String> {
    let mut app = start(Root::of::<DatePickerHarness>());
    app.wait_for("text:value:Choose a date")?;
    assert!(!app.screen().shows("label:Next month"), "closed at first");
    app.press("role:combobox")?;
    app.wait_for("label:2026-10-21")?;
    app.press("label:2026-10-21")?;
    app.wait_for("text:value:Oct 21, 2026")?;
    app.wait_for_gone("label:2026-10-21", WAIT)?;
    // The trigger shows the value; it opens again on Return.
    app.wait_for("text:Oct 21, 2026")?;
    app.press_key("enter")?;
    app.wait_for("label:Next month")?;
    app.press_key("escape")?;
    app.wait_for_gone("label:Next month", WAIT)?;
    Ok(())
}

// ---- List ---------------------------------------------------------------

#[component]
fn ListHarness() -> impl IntoElement {
    omarchy::gallery::pages::data::list()
}

#[test]
fn a_virtual_list_scrolls_and_keeps_its_selection_by_id() -> Result<(), String> {
    let mut app = start(Root::of::<ListHarness>());
    app.wait_for("label:Item 1")?;
    assert!(!app.screen().shows("Item 1500"), "far rows are not made");
    app.press("label:Item 3")?;
    app.wait_for("text:selected item 3")?;
    // 100 rows down (28 points each): row 101 at the top.
    app.scroll("id:list", px(2800.0))?;
    app.wait_for("label:Item 105")?;
    app.wait_for_gone("label:Item 3", WAIT)?;
    app.press("label:Item 105")?;
    app.wait_for("text:selected item 105")?;
    assert_eq!(app.screen().selected("label:Item 105"), Ok(true));
    // A recycled cell shows another row: not selected.
    assert_eq!(app.screen().selected("label:Item 104"), Ok(false));
    // Back up: row 3 is made again, unselected; the selection is still 105.
    app.scroll("id:list", px(-2800.0))?;
    app.wait_for("label:Item 3")?;
    assert_eq!(app.screen().selected("label:Item 3"), Ok(false));
    app.wait_for("text:selected item 105")?;
    Ok(())
}

// ---- Gallery pages ------------------------------------------------------

/// Each page of the group renders its demo and its state line.
macro_rules! page_renders {
    ($($test:ident: $page:ident => [$($sees:literal),*]),* $(,)?) => {$(
        #[test]
        fn $test() -> Result<(), String> {
            #[component]
            fn Page() -> impl IntoElement {
                omarchy::gallery::pages::data::$page()
            }
            let mut app = start(Root::of::<Page>());
            $(app.wait_for($sees)?;)*
            Ok(())
        }
    )*};
}

page_renders! {
    the_table_page_renders: table => ["text:selected pid 1207", "text:sort none, selected pid none"],
    the_tree_page_renders: tree => ["text:selected hypr, expanded config", "role:treeitem[hypr]"],
    the_calendar_page_renders: calendar => ["text:picked none", "text:range none"],
    the_date_picker_page_renders: date_picker => ["text:value Choose a date", "label:Trip dates"],
    the_color_picker_page_renders: color_picker => ["label:Hue", "label:Opacity", "text:color accent"],
    the_resizable_page_renders: resizable => ["text:Sidebar", "text:Terminal", "text:sizes as laid out"],
}
