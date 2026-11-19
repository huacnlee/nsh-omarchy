//! Headless tests of the navigation components: tabs, menus, accordion,
//! collapsible, pagination and sidebar, each placed by a small harness that
//! shows its state as text.

use std::time::Duration;

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Environment, Os, Points, Root, Size};
use nsh_omarchy::{
    NavSidebarGroup, NavSidebarItem, accordion_list, accordion_section, collapsible_section,
    menu_check_row, menu_context, menu_divider, menu_dropdown, menu_row, menu_shortcut,
    menu_trigger, nav_sidebar, nav_tab, nav_tab_panel, nav_tab_strip, nav_tabs, pagination_bar,
};
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

const TABS: [&str; 3] = ["Overview", "Activity", "Settings"];

#[component]
fn TabsHarness() -> impl IntoElement {
    let current = signal(0_usize);
    view()
        .child(
            nav_tabs().children(TABS.iter().enumerate().map(|(index, label)| {
                nav_tab(*label)
                    .selected(move || current.get() == index)
                    .on_press(move |_: &()| current.set(index))
            })),
        )
        .children(TABS.iter().enumerate().map(|(index, label)| {
            nav_tab_panel(move || current.get() == index).child(text(format!("Panel of {label}")))
        }))
        .child(text(move || format!("Current tab {}", TABS[current.get()])))
}

#[test]
fn a_tab_press_selects_it() -> Result<(), String> {
    let mut app = start(Root::of::<TabsHarness>());
    app.wait_for("Current tab Overview")?;
    assert!(app.screen().selected("role:tab[Overview]")?);
    app.press("Activity")?;
    app.wait_for("Current tab Activity")?;
    assert!(app.screen().selected("role:tab[Activity]")?);
    assert!(!app.screen().selected("role:tab[Overview]")?);
    Ok(())
}

#[test]
fn arrow_keys_move_between_tabs() -> Result<(), String> {
    let mut app = start(Root::of::<TabsHarness>());
    app.wait_for("Current tab Overview")?;
    app.press_key("tab")?;
    app.press_key("right")?;
    app.wait_for("Current tab Activity")?;
    app.press_key("end")?;
    app.wait_for("Current tab Settings")?;
    app.press_key("right")?;
    app.wait_for("Current tab Overview")?;
    Ok(())
}

#[component]
fn StripHarness() -> impl IntoElement {
    let selected = signal(0_u32);
    view()
        .child(
            nav_tab_strip()
                .labels(vec!["Day".into(), "Week".into(), "Month".into()])
                .selected(selected)
                .on_change(move |index: &u32| selected.set(*index)),
        )
        .child(text(move || format!("Strip index {}", selected.get())))
}

#[test]
fn a_tab_strip_reports_the_index() -> Result<(), String> {
    let mut app = start(Root::of::<StripHarness>());
    app.wait_for("Strip index 0")?;
    app.press("Month")?;
    app.wait_for("Strip index 2")?;
    assert!(app.screen().selected("role:tab[Month]")?);
    Ok(())
}

// ---- Menus ------------------------------------------------------------------

#[component]
fn MenuHarness() -> impl IntoElement {
    let last = signal(SharedString::from("nothing"));
    let wrap = signal(false);
    view()
        .child(
            menu_dropdown()
                .trigger(menu_trigger("File"))
                .child(
                    menu_row("New file")
                        .on_select(move || last.set("New file".into()))
                        .child(menu_shortcut("mod-n")),
                )
                .child(
                    menu_row("Save")
                        .disabled(true)
                        .on_select(move || last.set("Save".into())),
                )
                .child(menu_divider())
                .child(
                    menu_check_row("Word wrap", move || wrap.get())
                        .on_select(move || wrap.update(|on| *on = !*on)),
                ),
        )
        .child(text(move || format!("Chose {}", last.get())))
        .child(text(move || {
            format!("Wrap {}", if wrap.get() { "on" } else { "off" })
        }))
}

#[test]
fn a_menu_row_is_chosen_and_closes_the_menu() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("New file")?;
    assert!(app.shows(NEW_SHORTCUT));
    app.press("New file")?;
    app.wait_for("Chose New file")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    Ok(())
}

#[test]
fn a_disabled_menu_row_is_not_chosen() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("Save")?;
    assert!(!app.screen().enabled("role:menuitem[Save]")?);
    Ok(())
}

#[test]
fn a_check_row_toggles_and_keeps_the_menu_open() -> Result<(), String> {
    let mut app = start(Root::of::<MenuHarness>());
    app.wait_for("Chose nothing")?;
    app.press("File")?;
    app.wait_for("Word wrap")?;
    app.press("Word wrap")?;
    app.wait_for("Wrap on")?;
    assert!(app.shows("New file"), "the menu stays open");
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
    app.wait_for("New file")?;
    app.press_key("enter")?;
    app.wait_for("Chose New file")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    // Escape closes it without choosing.
    app.press("File")?;
    app.wait_for("New file")?;
    app.press_key("escape")?;
    app.wait_for_gone(NEW_SHORTCUT_TEXT, SHORT)?;
    assert!(app.shows("Chose New file"));
    Ok(())
}

#[component]
fn ContextHarness() -> impl IntoElement {
    let last = signal(SharedString::from("nothing"));
    view()
        .child(
            menu_context()
                .trigger(view().h(px(80.0)).child(text("Context area")))
                .child(menu_row("Copy").on_select(move || last.set("Copy".into()))),
        )
        .child(text(move || format!("Context chose {}", last.get())))
}

#[test]
fn a_right_click_opens_the_context_menu() -> Result<(), String> {
    let mut app = start(Root::of::<ContextHarness>());
    app.wait_for("Context chose nothing")?;
    app.right_click("Context area")?;
    app.wait_for("Copy")?;
    app.press("Copy")?;
    app.wait_for("Context chose Copy")?;
    Ok(())
}

// ---- Accordion and collapsible ----------------------------------------------

#[component]
fn AccordionHarness() -> impl IntoElement {
    let open = signal(Vec::<SharedString>::new());
    view()
        .child(
            accordion_list()
                .value(open)
                .on_change(move |next: &Vec<SharedString>| open.set(next.clone()))
                .child(accordion_section(
                    "one",
                    "First question",
                    text("First answer"),
                ))
                .child(accordion_section(
                    "two",
                    "Second question",
                    text("Second answer"),
                )),
        )
        .child(text(move || format!("Open [{}]", open.get().join(","))))
}

#[test]
fn an_accordion_section_toggles_one_at_a_time() -> Result<(), String> {
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
    Ok(())
}

#[component]
fn CollapsibleHarness() -> impl IntoElement {
    let open = signal(false);
    view()
        .child(collapsible_section(
            move || open.get(),
            "Advanced",
            move || open.update(|o| *o = !*o),
            text("Hidden details"),
        ))
        .child(text(move || format!("Collapsible {}", open.get())))
}

#[test]
fn a_collapsible_opens_and_closes() -> Result<(), String> {
    let mut app = start(Root::of::<CollapsibleHarness>());
    app.wait_for("Collapsible false")?;
    assert!(!app.shows("Hidden details"));
    app.press("label:Advanced")?;
    app.wait_for("Collapsible true")?;
    app.wait_for("Hidden details")?;
    app.press("label:Advanced")?;
    app.wait_for("Collapsible false")?;
    app.wait_for_gone("text:Hidden details", SHORT)?;
    Ok(())
}

// ---- Pagination -------------------------------------------------------------

#[component]
fn PaginationHarness() -> impl IntoElement {
    let page = signal(1_u32);
    view()
        .child(
            pagination_bar()
                .page(page)
                .total_pages(5)
                .on_change(move |p: &u32| page.set(*p)),
        )
        .child(text(move || format!("Showing page {}", page.get())))
}

#[test]
fn pagination_moves_by_page_and_step() -> Result<(), String> {
    let mut app = start(Root::of::<PaginationHarness>());
    app.wait_for("Showing page 1")?;
    assert!(!app.screen().enabled("label:Previous page")?);
    app.press("label:Next page")?;
    app.wait_for("Showing page 2")?;
    app.press("text:4")?;
    app.wait_for("Showing page 4")?;
    app.press("label:Previous page")?;
    app.wait_for("Showing page 3")?;
    app.press("text:5")?;
    app.wait_for("Showing page 5")?;
    assert!(!app.screen().enabled("label:Next page")?);
    Ok(())
}

// ---- Sidebar ----------------------------------------------------------------

#[component]
fn SidebarHarness() -> impl IntoElement {
    let current = signal(SharedString::from("inbox"));
    view()
        .child(
            nav_sidebar()
                .groups(vec![
                    NavSidebarGroup::new(
                        "Mail",
                        vec![
                            NavSidebarItem::new("inbox", "Inbox"),
                            NavSidebarItem::new("sent", "Sent"),
                        ],
                    ),
                    NavSidebarGroup::new(
                        "Labels",
                        vec![
                            NavSidebarItem::new("archive", "Archive").disabled(),
                            NavSidebarItem::new("work", "Work"),
                        ],
                    ),
                ])
                .value(current)
                .on_change(move |id: &SharedString| current.set(id.clone())),
        )
        .child(text(move || format!("Mailbox {}", current.get())))
}

#[test]
fn a_sidebar_row_press_selects_it() -> Result<(), String> {
    let mut app = start(Root::of::<SidebarHarness>());
    app.wait_for("Mailbox inbox")?;
    assert!(app.screen().selected("label:Inbox")?);
    assert!(app.shows("Labels"));
    app.press("label:Sent")?;
    app.wait_for("Mailbox sent")?;
    assert!(app.screen().selected("label:Sent")?);
    assert!(!app.screen().enabled("label:Archive")?);
    Ok(())
}

#[test]
fn sidebar_keys_skip_disabled_rows() -> Result<(), String> {
    let mut app = start(Root::of::<SidebarHarness>());
    app.wait_for("Mailbox inbox")?;
    app.press("label:Sent")?;
    app.wait_for("Mailbox sent")?;
    app.press_key("tab")?;
    app.press_key("tab")?;
    app.press_key("down")?;
    app.wait_for("Mailbox work")?;
    app.press_key("k")?;
    app.wait_for("Mailbox sent")?;
    app.press_key("home")?;
    app.wait_for("Mailbox inbox")?;
    Ok(())
}

// ---- Gallery pages ----------------------------------------------------------

#[test]
fn navigation_pages_are_interactive() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap();
    app.wait_for("Omarchy")?;
    app.press("label:Tabs")?;
    app.wait_for("text:tab: Overview")?;
    app.press("Activity")?;
    app.wait_for("text:tab: Activity")?;
    app.press("label:Pagination")?;
    app.wait_for("page 1 of 5")?;
    app.press("label:Next page >> nth=0")?;
    app.wait_for("page 2 of 5")?;
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
