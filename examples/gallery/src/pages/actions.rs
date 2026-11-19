//! The actions pages: buttons, toggles, groups and links.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, ButtonVariant, IconName, LinkTarget, tokens};

use crate::registry::Page;

// ---- Page parts -------------------------------------------------------------

/// A small heading over a demo.
fn heading(label: &'static str) -> impl IntoElement {
    text(label)
        .text_size(px(tokens::TEXT_SM))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(move || tokens::secondary())
}

/// A caption under a demo: what it does, or its state.
fn caption(label: impl IntoReactive<SharedString>) -> impl IntoElement {
    text(label)
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
}

/// A wrapping row of controls.
fn row() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(tokens::SPACE_2))
}

/// A demo: its heading and content.
fn section(title: &'static str) -> View {
    view().gap(px(tokens::SPACE_2)).child(heading(title))
}

fn page() -> View {
    view().gap(px(tokens::SPACE_6))
}

fn on_off(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

// ---- Button -----------------------------------------------------------------

pub fn button() -> AnyElement {
    let count = signal(0_u32);
    let pinned = signal(false);
    let add = move || count.update(|n| *n += 1);

    let mut appearance = section("Appearance");
    for (name, variant) in [
        ("Default", ButtonVariant::Default),
        ("Primary", ButtonVariant::Primary),
        ("Ghost", ButtonVariant::Ghost),
        ("Danger", ButtonVariant::Danger),
    ] {
        appearance = appearance.child(
            row()
                .child(view().w(px(72.0)).child(caption(name)))
                .child(om::button("Apply").variant(variant).on_press(add))
                .child(om::button("Unavailable").variant(variant).disabled(true)),
        );
    }

    page()
        .child(appearance)
        .child(
            section("Sizes").child(
                row()
                    .child(om::button("Small").primary().small().on_press(add))
                    .child(om::button("Medium").primary().medium().on_press(add))
                    .child(om::button("Large").primary().large().on_press(add)),
            ),
        )
        .child(
            section("Icons").child(
                row()
                    .child(om::button("Add workspace").icon(IconName::Plus).on_press(add))
                    .child(
                        om::icon_button(IconName::Plus, "Add panel")
                            .variant(ButtonVariant::Default)
                            .on_press(add),
                    )
                    .child(om::icon_button(IconName::Plus, "Add panel (small)").small().on_press(add))
                    .child(
                        om::icon_button(IconName::Plus, "Add panel unavailable")
                            .variant(ButtonVariant::Default)
                            .disabled(true),
                    )
                    .child(om::button("Saving").loading(true)),
            ),
        )
        .child(
            section("Selected").child(
                row()
                    .child(
                        om::button(move || {
                            SharedString::from(if pinned.get() { "Pinned" } else { "Pin" })
                        })
                        .ghost()
                        .selected(move || pinned.get())
                        .on_press(move || pinned.update(|p| *p = !*p)),
                    )
                    .child(caption(move || format!("Pinned: {}", on_off(pinned.get())))),
            ),
        )
        .child(
            row()
                .child(caption(move || format!("Activations: {}", count.get())))
                .child(om::button("Reset").on_press(move || count.set(0))),
        )
        .child(caption("Tab reaches each button; Space or Return presses it."))
        .into_element()
}

// ---- Toggle -----------------------------------------------------------------

pub fn toggle() -> AnyElement {
    let favorite = signal(false);
    let wrap = signal(true);
    let compact = signal(false);

    page()
        .child(
            section("Toggle").child(
                row()
                    .child(
                        om::toggle("Favorite panel")
                            .icon(IconName::Star)
                            .checked(move || favorite.get())
                            .on_change(move |on: &bool| favorite.set(*on)),
                    )
                    .child(caption(move || {
                        format!("Favorite panel: {}", on_off(favorite.get()))
                    })),
            ),
        )
        .child(
            section("Sizes").child(
                row()
                    .child(
                        om::toggle("Wrap lines")
                            .small()
                            .checked(move || wrap.get())
                            .on_change(move |on: &bool| wrap.set(*on)),
                    )
                    .child(
                        om::toggle("Compact rows")
                            .checked(move || compact.get())
                            .on_change(move |on: &bool| compact.set(*on)),
                    )
                    .child(om::toggle("Line numbers").large()),
            ),
        )
        .child(
            section("Disabled").child(
                row()
                    .child(om::toggle("Locked on").checked(true).disabled(true))
                    .child(om::toggle("Locked off").disabled(true)),
            ),
        )
        .child(caption("Space or Return turns the focused toggle on or off."))
        .into_element()
}

// ---- Toggle group -------------------------------------------------------------

const ARTICLES: [(&str, &str); 4] = [
    ("Autumn release notes", "draft"),
    ("Getting started", "review"),
    ("Keyboard shortcuts", "published"),
    ("Workspace migration", "draft"),
];

pub fn toggle_group() -> AnyElement {
    let filters = signal(vec![
        SharedString::from("draft"),
        "review".into(),
        "published".into(),
    ]);
    let shown = computed(move || {
        let on = filters.get();
        ARTICLES
            .iter()
            .filter(|(_, status)| on.iter().any(|s| s == status))
            .map(|(title, _)| *title)
            .collect::<Vec<_>>()
    });
    let formats = signal(Vec::<SharedString>::new());
    let align = signal(vec![SharedString::from("left")]);
    let has = move |values: Signal<Vec<SharedString>>, value: &str| {
        on_off(values.get().iter().any(|v| v == value))
    };

    page()
        .child(
            section("Article status")
                .child(
                    om::toggle_group()
                        .value(move || filters.get())
                        .on_change(move |next: &Vec<SharedString>| filters.set(next.clone()))
                        .child(om::toggle("Draft").value("draft"))
                        .child(om::toggle("In review").value("review"))
                        .child(om::toggle("Published").value("published")),
                )
                .child(caption(
                    "Select any combination. Turn every filter off to show no articles.",
                ))
                .child(caption(move || format!("{} articles", shown.with(Vec::len))))
                .child(
                    view()
                        .child(each(
                            shown,
                            |title: &&'static str| *title,
                            |title, _| {
                                text(move || title.get())
                                    .py(px(tokens::SPACE_2))
                                    .border_b_1()
                                    .border_color(move || tokens::divider())
                            },
                        ))
                        .child(when(
                            move || shown.with(Vec::is_empty),
                            || caption("No articles match."),
                        )),
                ),
        )
        .child(
            section("Formatting")
                .child(
                    om::toggle_group()
                        .small()
                        .value(move || formats.get())
                        .on_change(move |next: &Vec<SharedString>| formats.set(next.clone()))
                        .child(om::toggle("Bold").value("bold"))
                        .child(om::toggle("Italic").value("italic"))
                        .child(om::toggle("Underline").value("underline")),
                )
                .child(caption(move || {
                    format!(
                        "Bold {} · Italic {} · Underline {}",
                        has(formats, "bold"),
                        has(formats, "italic"),
                        has(formats, "underline")
                    )
                })),
        )
        .child(
            section("One at a time")
                .child(
                    om::toggle_group()
                        .single(true)
                        .value(move || align.get())
                        .on_change(move |next: &Vec<SharedString>| align.set(next.clone()))
                        .child(om::toggle("Align left").value("left"))
                        .child(om::toggle("Align center").value("center"))
                        .child(om::toggle("Align right").value("right")),
                )
                .child(caption(move || {
                    let align = align.get();
                    format!(
                        "Aligned: {}",
                        align.first().map_or("none", |a| a.as_str()).to_owned()
                    )
                })),
        )
        .child(caption(
            "Tab enters a group; Left / Right move, Space or Return toggles.",
        ))
        .into_element()
}

// ---- Button group -------------------------------------------------------------

const POSITIONS: [(&str, &str); 4] = [
    ("Top", "top"),
    ("Right", "right"),
    ("Bottom", "bottom"),
    ("Left", "left"),
];

pub fn button_group() -> AnyElement {
    let position = signal("top");
    let zoom = signal(100_i32);

    let mut positions = om::button_group();
    for (label, value) in POSITIONS {
        positions = positions.child(
            om::button(label)
                .selected(move || position.get() == value)
                .on_press(move || position.set(value)),
        );
    }

    page()
        .child(
            section("Toolbar position")
                .child(positions)
                .child(caption(move || {
                    format!("Toolbar is placed at the {}.", position.get())
                })),
        )
        .child(
            section("Zoom").child(
                row()
                    .child(
                        om::button_group()
                            .small()
                            .child(
                                om::icon_button(IconName::Minus, "Zoom out")
                                    .variant(ButtonVariant::Default)
                                    .on_press(move || zoom.update(|z| *z -= 10)),
                            )
                            .child(om::button("Reset zoom").on_press(move || zoom.set(100)))
                            .child(
                                om::icon_button(IconName::Plus, "Zoom in")
                                    .variant(ButtonVariant::Default)
                                    .on_press(move || zoom.update(|z| *z += 10)),
                            ),
                    )
                    .child(caption(move || format!("Zoom: {}%", zoom.get()))),
            ),
        )
        .child(
            section("Disabled").child(
                om::button_group()
                    .disabled(true)
                    .child(om::button("Cut"))
                    .child(om::button("Copy"))
                    .child(om::button("Paste")),
            ),
        )
        .child(caption("Each button is a stop of Tab and reports its own presses."))
        .into_element()
}

// ---- Link -------------------------------------------------------------------

pub fn link() -> AnyElement {
    let opened = signal(SharedString::from("nothing yet"));

    page()
        .child(
            section("External").child(
                row()
                    .child(om::link("Open Omarchy manual", "https://omarchy.org/manual"))
                    .child(caption("Opens in the system browser.")),
            ),
        )
        .child(
            section("In the app").child(
                row()
                    .child(
                        om::link("Release notes", "/releases")
                            .open_with(LinkTarget::App)
                            .on_press(move || opened.set("/releases".into())),
                    )
                    .child(caption(move || format!("Opened: {}", opened.get()))),
            ),
        )
        .into_element()
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Actions", id: "button", title: "Button", make: button },
    Page { group: "Actions", id: "toggle", title: "Toggle", make: toggle },
    Page { group: "Actions", id: "toggle_group", title: "Toggle group", make: toggle_group },
    Page { group: "Actions", id: "button_group", title: "Button group", make: button_group },
    Page { group: "Actions", id: "link", title: "Link", make: link },
];
