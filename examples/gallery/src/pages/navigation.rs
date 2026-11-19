//! The navigation pages: tabs, menus, accordions, collapsibles, pagination
//! and the sidebar. (No title bar page: it would take over the gallery
//! window's own bar.)

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, IconName, tokens};

use crate::registry::Page;

/// A titled demo block.
fn section(title: &'static str, body: impl IntoElement) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_2))
        .child(
            text(title)
                .text_size(px(tokens::TEXT_SM))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::secondary()),
        )
        .child(body)
        .into_element()
}

/// A line of secondary text that reports state.
fn status(line: impl Fn() -> String + 'static) -> Text {
    text(line)
        .text_size(px(tokens::TEXT_SM))
        .font_family(tokens::font_mono())
        .text_color(move || tokens::secondary())
}

fn page(sections: Vec<AnyElement>) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .max_w(px(640.0))
        .children(sections)
        .into_element()
}

fn on_off(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

pub fn tabs() -> AnyElement {
    const PANES: [(&str, &str, &str); 3] = [
        ("overview", "Overview", "3 services healthy, 1 deploying."),
        ("activity", "Activity", "12 deploys this week."),
        ("settings", "Settings", "Notifications and access."),
    ];
    let current = signal(SharedString::from("overview"));
    let underline = view()
        .gap(px(tokens::SPACE_3))
        .child(
            om::tabs()
                .value(move || current.get())
                .accessibility_label("Service")
                .on_change(move |value: &SharedString| current.set(value.clone()))
                .child(om::tab("overview", "Overview"))
                .child(om::tab("activity", "Activity").icon(IconName::Bell))
                .child(om::tab("settings", "Settings").icon(IconName::Settings))
                .child(om::tab("archived", "Archived").disabled(true)),
        )
        .child(dynamic(move || {
            let shown = current.get();
            let body = PANES.iter().find(|p| p.0 == shown.as_ref()).map_or("", |p| p.2);
            text(body).into_element()
        }))
        .child(status(move || format!("tab: {}", current.get())));

    // Left out, the value starts at the first tab.
    let range = signal(String::from("day"));
    let segmented = view()
        .gap(px(tokens::SPACE_3))
        .child(
            om::tabs()
                .segmented()
                .small()
                .accessibility_label("Range")
                .on_change(move |value: &SharedString| range.set(value.to_string()))
                .child(om::tab("day", "Day"))
                .child(om::tab("week", "Week"))
                .child(om::tab("month", "Month")),
        )
        .child(status(move || format!("range: {}", range.get())));

    page(vec![
        section("Underline: arrow keys move and select", underline),
        section("Segmented, small, keeping its own value", segmented),
    ])
}

pub fn menu() -> AnyElement {
    let last = signal(SharedString::from("nothing"));
    let sidebar = signal(true);
    let wrap = signal(false);
    let choose = move |what: &'static str| move || last.set(what.into());
    let file = om::menu()
        .trigger(om::button("File").icon(IconName::File))
        .child(om::menu_label("Document"))
        .child(om::menu_item("New file").shortcut("mod-n").on_select(choose("New file")))
        .child(om::menu_item("Open…").shortcut("mod-o").on_select(choose("Open")))
        .child(om::menu_item("Save").shortcut("mod-s").disabled(true))
        .child(om::menu_separator())
        .child(om::menu_label("View"))
        .child(
            om::menu_item("Show sidebar")
                .checked(move || sidebar.get())
                .on_select(move || sidebar.update(|on| *on = !*on)),
        )
        .child(
            om::menu_item("Word wrap")
                .checked(move || wrap.get())
                .on_select(move || wrap.update(|on| *on = !*on)),
        )
        .child(om::menu_separator())
        .child(
            om::menu_item("Delete")
                .icon(IconName::Trash)
                .danger(true)
                .on_select(choose("Delete")),
        );
    page(vec![section(
        "Return, Space or Down opens; arrows move; Escape closes",
        view()
            .gap(px(tokens::SPACE_3))
            .child(view().flex_row().child(file))
            .child(status(move || {
                format!(
                    "chose: {} · sidebar: {} · wrap: {}",
                    last.get(),
                    on_off(sidebar.get()),
                    on_off(wrap.get())
                )
            })),
    )])
}

pub fn context_menu() -> AnyElement {
    let last = signal(SharedString::from("nothing"));
    let choose = move |what: &'static str| move || last.set(what.into());
    let area = om::context_menu()
        .menu(
            view()
                .child(om::menu_item("Copy").icon(IconName::Copy).shortcut("mod-c").on_select(choose("Copy")))
                .child(om::menu_item("Paste").shortcut("mod-v").on_select(choose("Paste")))
                .child(om::menu_separator())
                .child(om::menu_item("Rename").icon(IconName::Pencil).shortcut("f2").on_select(choose("Rename")))
                .child(om::menu_item("Delete").shortcut("del").disabled(true)),
        )
        .child(
            view()
                .focusable(true)
                .label("Context area")
                .h(px(120.0))
                .items_center()
                .justify_center()
                .border_1()
                .border_color(move || tokens::control_border())
                .bg(move || tokens::normal_fill())
                .focus_visible(|s| s.border_color(move || tokens::focus_border()))
                .child(text("Right-click here (or shift-f10)").text_color(move || tokens::secondary())),
        );
    page(vec![section(
        "Context menu",
        view()
            .gap(px(tokens::SPACE_3))
            .child(area)
            .child(status(move || format!("chose: {}", last.get()))),
    )])
}

pub fn accordion() -> AnyElement {
    let single = signal(vec![SharedString::from("shipping")]);
    let many = signal(String::new());
    let faq = om::accordion()
        .value(move || single.get())
        .on_change(move |next: &Vec<SharedString>| single.set(next.clone()))
        .child(
            om::accordion_item("shipping", "How long does shipping take?")
                .child(text("Orders leave the warehouse within two business days.")),
        )
        .child(
            om::accordion_item("returns", "Can I return an item?")
                .child(text("Yes, within 30 days, unused and in its box.")),
        )
        .child(
            om::accordion_item("warranty", "Warranty (unavailable)")
                .disabled(true)
                .child(text("Two years.")),
        );
    let settings = om::accordion()
        .multiple(true)
        .on_change(move |next: &Vec<SharedString>| many.set(next.join(", ")))
        .child(om::accordion_item("display", "Display").child(text("Scale, resolution and night light.")))
        .child(om::accordion_item("sound", "Sound").child(text("Output device and volume.")));
    page(vec![
        section(
            "One at a time",
            view()
                .gap(px(tokens::SPACE_3))
                .child(faq)
                .child(status(move || format!("open: {}", single.get().join(", ")))),
        ),
        section(
            "Several open, keeping its own value",
            view()
                .gap(px(tokens::SPACE_3))
                .child(settings)
                .child(status(move || format!("several open: {}", many.get()))),
        ),
    ])
}

pub fn collapsible() -> AnyElement {
    let open = signal(false);
    let details = om::collapsible("Advanced options")
        .open(move || open.get())
        .on_open_change(move |next: &bool| open.set(*next))
        .child(
            view()
                .gap(px(tokens::SPACE_1))
                .child(text("Compression: zstd, level 3"))
                .child(text("Retries: 5, exponential backoff")),
        );
    page(vec![
        section(
            "Controlled",
            view()
                .gap(px(tokens::SPACE_3))
                .child(details)
                .child(status(move || {
                    format!("collapsible: {}", if open.get() { "open" } else { "closed" })
                })),
        ),
        section(
            "Keeping its own state, and disabled",
            view()
                .gap(px(tokens::SPACE_3))
                .child(om::collapsible("Release notes").child(text("0.1: the first release.")))
                .child(om::collapsible("Locked").disabled(true).child(text("Never shown."))),
        ),
    ])
}

pub fn pagination() -> AnyElement {
    let page_no = signal(1_u32);
    let wide = signal(7_u32);
    page(vec![
        section(
            "Five pages",
            view()
                .gap(px(tokens::SPACE_3))
                .child(om::pagination(move || page_no.get(), 5).on_change(move |p: &u32| page_no.set(*p)))
                .child(status(move || format!("page {} of 5", page_no.get()))),
        ),
        section(
            "Forty pages, seven shown",
            view()
                .gap(px(tokens::SPACE_3))
                .child(
                    om::pagination(move || wide.get(), 40)
                        .visible_pages(7)
                        .accessibility_label("Results")
                        .on_change(move |p: &u32| wide.set(*p)),
                )
                .child(status(move || format!("results page {} of 40", wide.get()))),
        ),
    ])
}

pub fn sidebar() -> AnyElement {
    let current = signal(SharedString::from("inbox"));
    let count = |n: &'static str| {
        text(n)
            .font_family(tokens::font_mono())
            .text_size(px(tokens::TEXT_XS))
            .text_color(move || tokens::secondary())
    };
    let nav = om::sidebar()
        .value(move || current.get())
        .accessibility_label("Mailboxes")
        .on_change(move |value: &SharedString| current.set(value.clone()))
        .header(
            text("Mail")
                .px(px(tokens::SPACE_3))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(move || tokens::bright()),
        )
        .footer(text("ada@example.com").px(px(tokens::SPACE_3)).text_size(px(tokens::TEXT_SM)))
        .child(
            om::sidebar_section("Mailboxes")
                .child(om::sidebar_item("inbox", "Inbox").icon(IconName::Inbox).suffix(count("12")))
                .child(om::sidebar_item("starred", "Starred").icon(IconName::Star))
                .child(om::sidebar_item("sent", "Sent").icon(IconName::Upload))
                .child(om::sidebar_item("drafts", "Drafts").icon(IconName::File).suffix(count("2"))),
        )
        .child(
            om::sidebar_section("Labels")
                .child(om::sidebar_item("work", "Work"))
                .child(om::sidebar_item("archive", "Archive").disabled(true))
                .child(om::sidebar_item("personal", "Personal")),
        )
        .h(px(360.0))
        .border_1();
    page(vec![section(
        "Focus a row, then Up/Down, Home/End",
        view()
            .flex_row()
            .gap(px(tokens::SPACE_4))
            .child(nav)
            .child(status(move || format!("mailbox: {}", current.get()))),
    )])
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Navigation", id: "tabs", title: "Tabs", make: tabs },
    Page { group: "Navigation", id: "menu", title: "Menu", make: menu },
    Page { group: "Navigation", id: "context_menu", title: "Context menu", make: context_menu },
    Page { group: "Navigation", id: "accordion", title: "Accordion", make: accordion },
    Page { group: "Navigation", id: "collapsible", title: "Collapsible", make: collapsible },
    Page { group: "Navigation", id: "pagination", title: "Pagination", make: pagination },
    Page { group: "Navigation", id: "sidebar", title: "Sidebar", make: sidebar },
];
