//! Navigation through the public API: tabs, the sidebar, accordions,
//! collapsibles, pagination, menus and the title bar, each placed by a
//! small harness that shows its state as text.

use std::time::Duration;

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Environment, Os, Points, Root, Size};
use nsh_omarchy::{self as om, IconName};
use nsh_testing::Headless;

/// A desktop-sized engine on macOS: menus open as popups, the secondary
/// button opens context menus, and Tab moves the keyboard focus.
fn start(root: Root) -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1000.0), Points(700.0)),
        environment: Environment {
            os: Os::MacOs,
            ..Environment::default()
        },
        ..EngineConfig::new(root)
    })
    .unwrap()
}

const SHORT: Duration = Duration::from_secs(2);

// ---- Tabs -------------------------------------------------------------------

#[component]
fn TabsHarness() -> impl IntoElement {
    let current = signal(SharedString::from("overview"));
    let asked = signal(0_u32);
    view()
        .child(
            om::tabs()
                .value(move || current.get())
                .accessibility_label("Sections")
                .on_change(move |value: &SharedString| {
                    asked.update(|n| *n += 1);
                    current.set(value.clone());
                })
                .child(om::tab("overview", "Overview"))
                .child(om::tab("activity", "Activity").icon(IconName::Star))
                .child(om::tab("archived", "Archived").disabled(true))
                .child(om::tab("settings", "Settings").suffix(text("3"))),
        )
        .child(text(move || format!("Current tab {} after {}", current.get(), asked.get())))
}

#[test]
fn a_tab_press_selects_it() -> Result<(), String> {
    let mut app = start(Root::of::<TabsHarness>());
    app.wait_for("Current tab overview after 0")?;
    assert!(app.screen().selected("role:tab[Overview]")?);
    assert!(app.shows("3"), "the suffix shows");
    app.press("Activity")?;
    app.wait_for("Current tab activity after 1")?;
    assert!(app.screen().selected("role:tab[Activity]")?);
    assert!(!app.screen().selected("role:tab[Overview]")?);
    // The selected tab asks for nothing.
    app.press("Activity")?;
    app.press("Settings")?;
    app.wait_for("Current tab settings after 2")?;
    assert!(!app.screen().enabled("role:tab[Archived]")?);
    Ok(())
}

#[test]
fn arrow_keys_move_between_tabs_past_disabled_ones() -> Result<(), String> {
    let mut app = start(Root::of::<TabsHarness>());
    app.wait_for("Current tab overview after 0")?;
    app.press_key("tab")?;
    app.press_key("right")?;
    app.wait_for("Current tab activity after 1")?;
    app.press_key("right")?;
    app.wait_for("Current tab settings after 2")?;
    app.press_key("home")?;
    app.wait_for("Current tab overview after 3")?;
    Ok(())
}

#[component]
fn OwnTabsHarness() -> impl IntoElement {
    let last = signal(SharedString::from("none"));
    view()
        .child(
            om::tabs()
                .segmented()
                .small()
                .on_change(move |value: &SharedString| last.set(value.clone()))
                .child(om::tab("day", "Day").disabled(true))
                .child(om::tab("week", "Week"))
                .child(om::tab("month", "Month")),
        )
        .child(text(move || format!("Asked {}", last.get())))
}

#[test]
fn tabs_without_a_value_keep_their_own() -> Result<(), String> {
    let mut app = start(Root::of::<OwnTabsHarness>());
    app.wait_for("Asked none")?;
    assert!(app.screen().selected("role:tab[Week]")?, "the first enabled tab");
    app.press("Month")?;
    app.wait_for("Asked month")?;
    assert!(app.screen().selected("role:tab[Month]")?);
    assert!(!app.screen().selected("role:tab[Week]")?);
    Ok(())
}

// ---- Sidebar ----------------------------------------------------------------

#[component]
fn SidebarHarness() -> impl IntoElement {
    let current = signal(SharedString::from("inbox"));
    view()
        .child(
            om::sidebar()
                .value(move || current.get())
                .accessibility_label("Mailboxes")
                .header(text("Mail app"))
                .footer(text("Signed in"))
                .on_change(move |value: &SharedString| current.set(value.clone()))
                .child(
                    om::sidebar_section("Mail")
                        .child(om::sidebar_item("inbox", "Inbox").icon(IconName::Inbox).suffix(text("12")))
                        .child(om::sidebar_item("sent", "Sent")),
                )
                .child(
                    om::sidebar_section("Labels")
                        .child(om::sidebar_item("archive", "Archive").disabled(true))
                        .child(om::sidebar_item("work", "Work")),
                ),
        )
        .child(text(move || format!("Mailbox {}", current.get())))
}

#[test]
fn a_sidebar_item_press_selects_it() -> Result<(), String> {
    let mut app = start(Root::of::<SidebarHarness>());
    app.wait_for("Mailbox inbox")?;
    for shown in ["Mail app", "Signed in", "Mail", "Labels", "12"] {
        assert!(app.shows(shown), "{shown} shows");
    }
    assert!(app.screen().selected("label:Inbox")?);
    app.press("label:Sent")?;
    app.wait_for("Mailbox sent")?;
    assert!(app.screen().selected("label:Sent")?);
    assert!(!app.screen().enabled("label:Archive")?);
    Ok(())
}

#[test]
fn sidebar_keys_skip_disabled_items() -> Result<(), String> {
    let mut app = start(Root::of::<SidebarHarness>());
    app.wait_for("Mailbox inbox")?;
    app.press_key("tab")?;
    app.press_key("down")?;
    app.wait_for("Mailbox sent")?;
    app.press_key("down")?;
    app.wait_for("Mailbox work")?;
    app.press_key("up")?;
    app.wait_for("Mailbox sent")?;
    app.press_key("home")?;
    app.wait_for("Mailbox inbox")?;
    Ok(())
}

// ---- Accordion and collapsible ----------------------------------------------

#[component]
fn AccordionHarness() -> impl IntoElement {
    let open = signal(Vec::<SharedString>::new());
    view()
        .child(
            om::accordion()
                .value(move || open.get())
                .on_change(move |next: &Vec<SharedString>| open.set(next.clone()))
                .child(om::accordion_item("one", "First question").child(text("First answer")))
                .child(om::accordion_item("two", "Second question").child(text("Second answer")))
                .child(om::accordion_item("three", "Locked question").disabled(true)),
        )
        .child(text(move || format!("Open [{}]", open.get().join(","))))
}

#[test]
fn an_accordion_opens_one_item_at_a_time() -> Result<(), String> {
    let mut app = start(Root::of::<AccordionHarness>());
    app.wait_for("Open []")?;
    app.press("role:button[First question]")?;
    app.wait_for("Open [one]")?;
    app.wait_for("First answer")?;
    app.press("role:button[Second question]")?;
    app.wait_for("Open [two]")?;
    app.wait_for("Second answer")?;
    app.press("role:button[Second question]")?;
    app.wait_for("Open []")?;
    assert!(!app.screen().enabled("role:button[Locked question]")?);
    Ok(())
}

#[component]
fn OwnAccordionHarness() -> impl IntoElement {
    let last = signal(String::from("-"));
    view()
        .child(
            om::accordion()
                .multiple(true)
                .on_change(move |next: &Vec<SharedString>| last.set(next.join(",")))
                .child(om::accordion_item("display", "Display").child(text("Scale")))
                .child(om::accordion_item("sound", "Sound").child(text("Volume"))),
        )
        .child(text(move || format!("Asked [{}]", last.get())))
}

#[test]
fn an_accordion_without_a_value_keeps_several_open() -> Result<(), String> {
    let mut app = start(Root::of::<OwnAccordionHarness>());
    app.wait_for("Asked [-]")?;
    app.press("role:button[Display]")?;
    app.wait_for("Asked [display]")?;
    app.press("role:button[Sound]")?;
    app.wait_for("Asked [display,sound]")?;
    app.wait_for("Scale")?;
    app.wait_for("Volume")?;
    Ok(())
}

#[component]
fn CollapsibleHarness() -> impl IntoElement {
    let open = signal(false);
    let asked = signal(String::from("-"));
    view()
        .child(
            om::collapsible("Advanced")
                .open(move || open.get())
                .on_open_change(move |next: &bool| open.set(*next))
                .child(text("Hidden details")),
        )
        .child(
            om::collapsible("Own")
                .on_open_change(move |next: &bool| asked.set(next.to_string()))
                .child(text("Own details")),
        )
        .child(om::collapsible("Locked").disabled(true).child(text("Locked details")))
        .child(text(move || format!("Collapsible {} own {}", open.get(), asked.get())))
}

#[test]
fn a_collapsible_opens_and_closes() -> Result<(), String> {
    let mut app = start(Root::of::<CollapsibleHarness>());
    app.wait_for("Collapsible false own -")?;
    assert!(!app.shows("Hidden details"));
    app.press("label:Advanced")?;
    app.wait_for("Collapsible true own -")?;
    app.wait_for("Hidden details")?;
    app.press("label:Advanced")?;
    app.wait_for("Collapsible false own -")?;
    app.wait_for_gone("text:Hidden details", SHORT)?;
    app.press("label:Own")?;
    app.wait_for("Collapsible false own true")?;
    app.wait_for("Own details")?;
    assert!(!app.screen().enabled("label:Locked")?);
    Ok(())
}

// ---- Pagination -------------------------------------------------------------

#[component]
fn PaginationHarness() -> impl IntoElement {
    let page = signal(1_u32);
    let asked = signal(0_u32);
    view()
        .child(
            om::pagination(move || page.get(), 5)
                .accessibility_label("Results")
                .on_change(move |p: &u32| page.set(*p)),
        )
        .child(text(move || format!("Showing page {}", page.get())))
        .child(om::pagination(1, 3).on_change(move |p: &u32| asked.set(*p)))
        .child(text(move || format!("Asked page {}", asked.get())))
}

#[test]
fn pagination_moves_by_page_and_step() -> Result<(), String> {
    let mut app = start(Root::of::<PaginationHarness>());
    app.wait_for("Showing page 1")?;
    assert!(!app.screen().enabled("label:Previous page >> nth=0")?);
    app.press("label:Next page >> nth=0")?;
    app.wait_for("Showing page 2")?;
    app.press("text:4")?;
    app.wait_for("Showing page 4")?;
    app.press("label:Previous page >> nth=0")?;
    app.wait_for("Showing page 3")?;
    app.press("text:5")?;
    app.wait_for("Showing page 5")?;
    assert!(!app.screen().enabled("label:Next page >> nth=0")?);
    // A page given as a value moves on its own.
    app.press("label:Next page >> nth=1")?;
    app.wait_for("Asked page 2")?;
    app.press("label:Next page >> nth=1")?;
    app.wait_for("Asked page 3")?;
    Ok(())
}

// ---- Menus ------------------------------------------------------------------

#[component]
fn MenuHarness() -> impl IntoElement {
    let last = signal(SharedString::from("nothing"));
    let wrap = signal(false);
    let new_file = move || last.set("New".into());
    view()
        .child(
            // api.md's example.
            om::menu()
                .trigger(om::button("File"))
                .child(om::menu_item("New").shortcut("mod-n").on_select(new_file))
                .child(om::menu_item("Save").disabled(true).on_select(move || last.set("Save".into())))
                .child(om::menu_separator())
                .child(om::menu_label("View"))
                .child(
                    om::menu_item("Word wrap")
                        .checked(move || wrap.get())
                        .on_select(move || wrap.update(|on| *on = !*on)),
                )
                .child(
                    om::menu_item("Delete")
                        .icon(IconName::Trash)
                        .danger(true)
                        .on_select(move || last.set("Delete".into())),
                ),
        )
        .child(text(move || format!("Chose {}", last.get())))
        .child(text(move || format!("Wrap {}", if wrap.get() { "on" } else { "off" })))
}

#[test]
fn a_menu_item_is_chosen_and_closes_the_menu() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("New")?;
    assert!(app.shows(NEW_SHORTCUT));
    assert!(app.shows("View"));
    app.press("text:New")?;
    app.wait_for("Chose New")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    app.press("File")?;
    app.wait_for("Delete")?;
    app.press("text:Delete")?;
    app.wait_for("Chose Delete")?;
    Ok(())
}

#[test]
fn a_disabled_menu_item_is_not_chosen() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("Save")?;
    assert!(!app.screen().enabled("role:menuitem[Save]")?);
    Ok(())
}

#[test]
fn a_checked_item_toggles_and_keeps_the_menu_open() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("Word wrap")?;
    app.press("Word wrap")?;
    app.wait_for("Wrap on")?;
    assert!(app.shows(NEW_SHORTCUT), "the menu stays open");
    app.press("Word wrap")?;
    app.wait_for("Wrap off")?;
    Ok(())
}

#[test]
fn the_keyboard_opens_the_menu_and_chooses() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press_key("tab")?;
    app.press_key("down")?;
    app.wait_for(NEW_SHORTCUT)?;
    app.press_key("enter")?;
    app.wait_for("Chose New")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    app.press("File")?;
    app.wait_for(NEW_SHORTCUT)?;
    app.press_key("escape")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    assert!(app.shows("Chose New"));
    Ok(())
}

#[component]
fn ControlledMenuHarness() -> impl IntoElement {
    let open = signal(false);
    view()
        .child(
            om::menu()
                .trigger(om::button("Edit"))
                .open(move || open.get())
                .on_open_change(move |next: &bool| open.set(*next))
                .child(om::menu_item("Undo").shortcut("mod-z")),
        )
        .child(om::button("Open it").on_press(move || open.set(true)))
        .child(text(move || format!("Menu {}", if open.get() { "open" } else { "closed" })))
}

#[test]
fn a_controlled_menu_follows_open() -> Result<(), String> {
    let mut app = start(Root::of::<ControlledMenuHarness>());
    app.wait_for("Menu closed")?;
    app.press("Open it")?;
    app.wait_for("Undo")?;
    app.press_key("escape")?;
    app.wait_for("Menu closed")?;
    app.wait_for_gone("text:Undo", SHORT)?;
    app.press("Edit")?;
    app.wait_for("Menu open")?;
    app.wait_for("Undo")?;
    Ok(())
}

#[component]
fn ContextHarness() -> impl IntoElement {
    let last = signal(SharedString::from("nothing"));
    view()
        .child(
            om::context_menu()
                .menu(
                    view()
                        .child(om::menu_item("Copy").on_select(move || last.set("Copy".into())))
                        .child(om::menu_item("Paste").on_select(move || last.set("Paste".into()))),
                )
                .child(view().h(px(80.0)).child(text("Context area"))),
        )
        .child(
            om::context_menu()
                .disabled(true)
                .menu(om::menu_item("Never"))
                .child(view().h(px(80.0)).child(text("Locked area"))),
        )
        .child(text(move || format!("Context chose {}", last.get())))
}

#[test]
fn a_right_click_opens_the_context_menu() -> Result<(), String> {
    let mut app = start(Root::of::<ContextHarness>());
    app.wait_for("Context chose nothing")?;
    app.right_click("Context area")?;
    app.wait_for("Paste")?;
    app.press("Paste")?;
    app.wait_for("Context chose Paste")?;
    app.right_click("Locked area")?;
    app.wait(Duration::from_millis(300))?;
    assert!(!app.shows("Never"));
    Ok(())
}

// ---- Title bar --------------------------------------------------------------

#[component]
fn TitleBarHarness() -> impl IntoElement {
    let shared = signal(0_u32);
    view()
        .child(
            om::title_bar()
                .title("Files")
                .child(om::button("Share").on_press(move || shared.update(|n| *n += 1))),
        )
        .child(text(move || format!("Shared {}", shared.get())))
}

#[test]
fn the_title_bar_shows_its_title_and_children() -> Result<(), String> {
    let mut app = start(Root::of::<TitleBarHarness>());
    app.wait_for("Files")?;
    app.press("Share")?;
    app.wait_for("Shared 1")?;
    Ok(())
}

/// The menu's `mod-n` as the host platform writes it.
const NEW_SHORTCUT: &str = if cfg!(target_os = "macos") {
    "cmd-n"
} else {
    "ctrl-n"
};
const NEW_SHORTCUT_TEXT: &str = if cfg!(target_os = "macos") {
    "text:cmd-n"
} else {
    "text:ctrl-n"
};
