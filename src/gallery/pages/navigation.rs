//! Gallery pages of the `navigation` group. Each `pub fn` is one page.

use nsh_core::prelude::*;

use crate::components::{
    NavSidebarGroup, NavSidebarItem, accordion_list, accordion_section, collapsible_section,
    menu_check_row, menu_context, menu_divider, menu_dropdown, menu_heading, menu_row,
    menu_shortcut, menu_trigger, nav_sidebar, nav_tab, nav_tab_panel, nav_tab_strip, nav_tabs,
    pagination_bar,
};
use crate::tokens;

/// A titled demo block.
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

pub fn tabs() -> AnyElement {
    const PANES: [(&str, &str); 3] = [
        ("Overview", "Overview: 3 services healthy, 1 deploying."),
        ("Activity", "Activity: 12 deploys this week."),
        ("Settings", "Settings: notifications and access."),
    ];
    let current = signal(0_usize);
    let underline = view()
        .gap(px(tokens::SPACE_3))
        .child(
            nav_tabs()
                .children(PANES.iter().enumerate().map(|(index, (label, _))| {
                    nav_tab(*label)
                        .selected(move || current.get() == index)
                        .on_press(move |_: &()| current.set(index))
                }))
                .child(nav_tab("Archived").disabled(true)),
        )
        .children(PANES.iter().enumerate().map(|(index, (_, body))| {
            nav_tab_panel(move || current.get() == index).child(text(*body))
        }))
        .child(status(move || format!("tab: {}", PANES[current.get()].0)));

    let segment = signal(1_u32);
    let segmented = view()
        .gap(px(tokens::SPACE_3))
        .child(
            nav_tab_strip()
                .labels(vec!["Day".into(), "Week".into(), "Month".into()])
                .segmented(true)
                .selected(segment)
                .on_change(move |index: &u32| segment.set(*index)),
        )
        .child(status(move || {
            format!(
                "range: {}",
                ["Day", "Week", "Month"][segment.get() as usize]
            )
        }));

    page(vec![
        section("Plain: arrow keys move and select", underline).into_element(),
        section("Segmented", segmented).into_element(),
    ])
}

pub fn menu() -> AnyElement {
    let last = signal(SharedString::from("nothing"));
    let sidebar = signal(true);
    let wrap = signal(false);
    let choose = move |what: &'static str| move || last.set(what.into());
    let file = menu_dropdown()
        .trigger(menu_trigger("File"))
        .child(
            menu_row("New file")
                .on_select(choose("New file"))
                .child(menu_shortcut("Ctrl N")),
        )
        .child(
            menu_row("Open…")
                .on_select(choose("Open"))
                .child(menu_shortcut("Ctrl O")),
        )
        .child(
            menu_row("Save")
                .disabled(true)
                .child(menu_shortcut("Ctrl S")),
        )
        .child(menu_divider())
        .child(menu_heading("View"))
        .child(
            menu_check_row("Show sidebar", move || sidebar.get())
                .on_select(move || sidebar.update(|on| *on = !*on)),
        )
        .child(
            menu_check_row("Word wrap", move || wrap.get())
                .on_select(move || wrap.update(|on| *on = !*on)),
        )
        .child(menu_divider())
        .child(
            menu_row("Quit")
                .on_select(choose("Quit"))
                .child(menu_shortcut("Ctrl Q")),
        );
    page(vec![
        section(
            "Dropdown: Return, Space or Down opens; arrows move; Escape closes",
            view()
                .gap(px(tokens::SPACE_3))
                .child(file)
                .child(status(move || {
                    format!(
                        "chose: {} · sidebar: {} · wrap: {}",
                        last.get(),
                        if sidebar.get() { "on" } else { "off" },
                        if wrap.get() { "on" } else { "off" }
                    )
                })),
        )
        .into_element(),
    ])
}

pub fn context_menu() -> AnyElement {
    let last = signal(SharedString::from("nothing"));
    let choose = move |what: &'static str| move || last.set(what.into());
    let area = menu_context()
        .trigger(
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
                .child(
                    text("Right-click here (or Shift+F10)").text_color(move || tokens::secondary()),
                ),
        )
        .child(
            menu_row("Copy")
                .on_select(choose("Copy"))
                .child(menu_shortcut("Ctrl C")),
        )
        .child(
            menu_row("Paste")
                .on_select(choose("Paste"))
                .child(menu_shortcut("Ctrl V")),
        )
        .child(menu_divider())
        .child(
            menu_row("Rename")
                .on_select(choose("Rename"))
                .child(menu_shortcut("F2")),
        )
        .child(
            menu_row("Delete")
                .disabled(true)
                .child(menu_shortcut("Del")),
        );
    page(vec![
        section(
            "Context menu",
            view()
                .gap(px(tokens::SPACE_3))
                .child(area)
                .child(status(move || format!("chose: {}", last.get()))),
        )
        .into_element(),
    ])
}

pub fn accordion() -> AnyElement {
    let single = signal(vec![SharedString::from("shipping")]);
    let many = signal(Vec::<SharedString>::new());
    let faq = accordion_list()
        .value(single)
        .on_change(move |next: &Vec<SharedString>| single.set(next.clone()))
        .child(accordion_section(
            "shipping",
            "How long does shipping take?",
            text("Orders leave the warehouse within two business days."),
        ))
        .child(accordion_section(
            "returns",
            "Can I return an item?",
            text("Yes, within 30 days, unused and in its box."),
        ))
        .child(
            accordion_section("warranty", "Warranty (unavailable)", text("Two years."))
                .disabled(true),
        );
    let settings = accordion_list()
        .multiple(true)
        .value(many)
        .on_change(move |next: &Vec<SharedString>| many.set(next.clone()))
        .child(accordion_section(
            "display",
            "Display",
            text("Scale, resolution and night light."),
        ))
        .child(accordion_section(
            "sound",
            "Sound",
            text("Output device and volume."),
        ));
    page(vec![
        section(
            "One at a time",
            view()
                .gap(px(tokens::SPACE_3))
                .child(faq)
                .child(status(move || format!("open: {}", single.get().join(", ")))),
        )
        .into_element(),
        section(
            "Several open",
            view()
                .gap(px(tokens::SPACE_3))
                .child(settings)
                .child(status(move || format!("open: {}", many.get().join(", ")))),
        )
        .into_element(),
    ])
}

pub fn collapsible() -> AnyElement {
    let open = signal(false);
    let details = collapsible_section(
        move || open.get(),
        "Advanced options",
        move || open.update(|o| *o = !*o),
        view()
            .gap(px(tokens::SPACE_1))
            .child(text("Compression: zstd, level 3"))
            .child(text("Retries: 5, exponential backoff")),
    );
    page(vec![
        section(
            "Controlled collapsible",
            view()
                .gap(px(tokens::SPACE_3))
                .child(details)
                .child(status(move || {
                    format!(
                        "collapsible: {}",
                        if open.get() { "open" } else { "closed" }
                    )
                })),
        )
        .into_element(),
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
                .child(
                    pagination_bar()
                        .page(page_no)
                        .total_pages(5)
                        .on_change(move |p: &u32| page_no.set(*p)),
                )
                .child(status(move || format!("page {} of 5", page_no.get()))),
        )
        .into_element(),
        section(
            "Forty pages, seven shown",
            view()
                .gap(px(tokens::SPACE_3))
                .child(
                    pagination_bar()
                        .page(wide)
                        .total_pages(40)
                        .visible_pages(7)
                        .on_change(move |p: &u32| wide.set(*p)),
                )
                .child(status(move || format!("results page {} of 40", wide.get()))),
        )
        .into_element(),
    ])
}

pub fn sidebar() -> AnyElement {
    let current = signal(SharedString::from("inbox"));
    let groups = vec![
        NavSidebarGroup::new(
            "Mail",
            vec![
                NavSidebarItem::new("inbox", "Inbox").badge("12"),
                NavSidebarItem::new("starred", "Starred"),
                NavSidebarItem::new("sent", "Sent"),
                NavSidebarItem::new("drafts", "Drafts").badge("2"),
            ],
        ),
        NavSidebarGroup::new(
            "Labels",
            vec![
                NavSidebarItem::new("work", "Work"),
                NavSidebarItem::new("archive", "Archive").disabled(),
                NavSidebarItem::new("personal", "Personal"),
            ],
        ),
    ];
    let nav = nav_sidebar()
        .groups(groups)
        .value(current)
        .accessibility_label("Mailboxes")
        .on_change(move |id: &SharedString| current.set(id.clone()))
        .w(px(200.0))
        .border_1()
        .border_color(move || tokens::border())
        .bg(move || tokens::surface());
    page(vec![
        section(
            "Focus a row, then Up/Down, j/k, Home/End",
            view()
                .flex_row()
                .gap(px(tokens::SPACE_4))
                .child(nav)
                .child(status(move || format!("mailbox: {}", current.get()))),
        )
        .into_element(),
    ])
}
