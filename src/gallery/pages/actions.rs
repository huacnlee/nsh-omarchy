//! Gallery pages of the `actions` group. Each `pub fn` is one page.

use nsh_base::LinkOpenWith;
use nsh_core::prelude::*;

use crate::components::{self as c, ButtonGroupItem, ButtonSize, ButtonVariant};
use crate::tokens;

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

/// A glyph standing in for an icon.
fn glyph(symbol: &'static str) -> impl IntoElement {
    text(symbol)
        .font_family(move || tokens::font_mono())
        .text_size(px(tokens::TEXT_LG))
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
                .child(c::button_variant("Apply", variant, ButtonSize::Md).on_press(add))
                .child(c::button_variant("Unavailable", variant, ButtonSize::Md).disabled(true)),
        );
    }

    view()
        .gap(px(tokens::SPACE_6))
        .child(appearance)
        .child(
            section("Sizes").child(
                row()
                    .child(
                        c::button_variant("Small", ButtonVariant::Primary, ButtonSize::Sm)
                            .on_press(add),
                    )
                    .child(
                        c::button_variant("Medium", ButtonVariant::Primary, ButtonSize::Md)
                            .on_press(add),
                    )
                    .child(
                        c::button_variant("Large", ButtonVariant::Primary, ButtonSize::Lg)
                            .on_press(add),
                    ),
            ),
        )
        .child(
            section("Icons").child(
                row()
                    .child(
                        c::button_with_icon(
                            glyph("+"),
                            "Add workspace",
                            ButtonVariant::Default,
                            ButtonSize::Md,
                        )
                        .on_press(add),
                    )
                    .child(
                        c::icon_button(
                            glyph("+"),
                            "Add panel",
                            ButtonVariant::Default,
                            ButtonSize::Md,
                        )
                        .on_press(add),
                    )
                    .child(
                        c::icon_button(
                            glyph("+"),
                            "Add panel (small)",
                            ButtonVariant::Ghost,
                            ButtonSize::Sm,
                        )
                        .on_press(add),
                    )
                    .child(
                        c::icon_button(
                            glyph("+"),
                            "Add panel unavailable",
                            ButtonVariant::Default,
                            ButtonSize::Md,
                        )
                        .disabled(true),
                    ),
            ),
        )
        .child(
            section("Selected").child(
                row()
                    .child(
                        c::button_ghost(move || {
                            SharedString::from(if pinned.get() { "Pinned" } else { "Pin" })
                        })
                        .selected(pinned)
                        .on_press(move || pinned.update(|p| *p = !*p)),
                    )
                    .child(caption(move || format!("Pinned: {}", on_off(pinned.get())))),
            ),
        )
        .child(
            row()
                .child(caption(move || format!("Activations: {}", count.get())))
                .child(c::button("Reset").on_press(move || count.set(0))),
        )
        .child(caption(
            "Tab reaches each button; Space or Return presses it.",
        ))
        .into_element()
}

// ---- Toggle -----------------------------------------------------------------

pub fn toggle() -> AnyElement {
    let favorite = signal(false);
    let wrap = signal(true);
    let compact = signal(false);
    let bold = signal(false);

    view()
        .gap(px(tokens::SPACE_6))
        .child(
            section("Toggle").child(
                row()
                    .child(
                        c::toggle_with_icon(glyph("*"), "Favorite panel", ButtonSize::Md)
                            .pressed(favorite)
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
                        c::toggle_sized("Wrap lines", ButtonSize::Sm)
                            .pressed(wrap)
                            .on_change(move |on: &bool| wrap.set(*on)),
                    )
                    .child(
                        c::toggle("Compact rows")
                            .pressed(compact)
                            .on_change(move |on: &bool| compact.set(*on)),
                    )
                    .child(
                        c::icon_toggle(glyph("B"), "Bold", ButtonSize::Lg)
                            .pressed(bold)
                            .on_change(move |on: &bool| bold.set(*on)),
                    ),
            ),
        )
        .child(
            section("Disabled").child(
                row()
                    .child(c::toggle("Locked on").pressed(true).disabled(true))
                    .child(c::toggle("Locked off").disabled(true)),
            ),
        )
        .child(caption(
            "Space or Return turns the focused toggle on or off.",
        ))
        .into_element()
}

// ---- Toggle group -------------------------------------------------------------

const ARTICLES: [(&str, usize); 4] = [
    ("Autumn release notes", 0),
    ("Getting started", 1),
    ("Keyboard shortcuts", 2),
    ("Workspace migration", 0),
];

pub fn toggle_group() -> AnyElement {
    let filters = signal([true, true, true]);
    let shown = computed(move || {
        let on = filters.get();
        ARTICLES
            .iter()
            .filter(|(_, status)| on[*status])
            .map(|(title, _)| *title)
            .collect::<Vec<_>>()
    });

    let status = ["Draft", "In review", "Published"]
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            c::toggle(label)
                .pressed(move || filters.get()[index])
                .on_change(move |on: &bool| filters.update(|f| f[index] = *on))
        });

    let formats = signal([false, false, false]);
    let format = [("B", "Bold"), ("I", "Italic"), ("U", "Underline")]
        .into_iter()
        .enumerate()
        .map(|(index, (symbol, label))| {
            c::icon_toggle(glyph(symbol), label, ButtonSize::Md)
                .pressed(move || formats.get()[index])
                .on_change(move |on: &bool| formats.update(|f| f[index] = *on))
        });

    view()
        .gap(px(tokens::SPACE_6))
        .child(
            section("Article status")
                .child(c::toggle_group("Article status filters", status))
                .child(caption(
                    "Select any combination. Turn every filter off to show no articles.",
                ))
                .child(caption(move || {
                    format!("{} articles", shown.with(Vec::len))
                }))
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
                .child(c::toggle_group("Formatting", format))
                .child(caption(move || {
                    let f = formats.get();
                    format!(
                        "Bold {} · Italic {} · Underline {}",
                        on_off(f[0]),
                        on_off(f[1]),
                        on_off(f[2])
                    )
                })),
        )
        .child(caption(
            "Tab enters the group; Left / Right move, Space or Return toggles.",
        ))
        .into_element()
}

// ---- Button group -------------------------------------------------------------

const POSITIONS: [&str; 4] = ["Top", "Right", "Bottom", "Left"];
const SECTIONS: [&str; 3] = ["Overview", "Activity", "Settings"];

pub fn button_group() -> AnyElement {
    let position = signal(Some(0_usize));
    let tab = signal(Some(0_usize));

    view()
        .gap(px(tokens::SPACE_6))
        .child(
            section("Toolbar position")
                .child(
                    c::button_group("Toolbar position", POSITIONS, position, move |i| {
                        position.set(Some(i))
                    })
                    .max_w(px(360.0)),
                )
                .child(caption(move || {
                    let at = position.get().map_or("nowhere", |i| POSITIONS[i]);
                    format!("Toolbar is placed at the {}.", at.to_lowercase())
                }))
                .child(caption(
                    "Left / Right move the focus · Return / Space choose",
                )),
        )
        .child(
            section("With a disabled choice").child(
                c::button_group(
                    "Contrast",
                    [
                        "Auto".into(),
                        "Light".into(),
                        ButtonGroupItem::new("High contrast").disabled(true),
                    ],
                    Some(0),
                    |_| {},
                )
                .max_w(px(360.0)),
            ),
        )
        .child(
            section("Tab list")
                .child(c::tab_list(
                    "Workspace sections",
                    [
                        ButtonGroupItem::new("Overview"),
                        "Activity".into(),
                        "Settings".into(),
                        ButtonGroupItem::new("Archived").disabled(true),
                    ],
                    tab,
                    move |i| tab.set(Some(i)),
                ))
                .child(
                    view()
                        .p(px(tokens::SPACE_3))
                        .border_1()
                        .border_color(move || tokens::divider())
                        .child(text(move || {
                            let name = tab.get().map_or("Nothing", |i| SECTIONS[i]);
                            format!("{name} panel")
                        })),
                )
                .child(caption("Arrows move between tabs and select as they go.")),
        )
        .into_element()
}

// ---- Link -------------------------------------------------------------------

pub fn link() -> AnyElement {
    let opened = signal(SharedString::from("nothing yet"));

    view()
        .gap(px(tokens::SPACE_6))
        .child(
            section("External").child(
                row()
                    .child(
                        c::link("Open Omarchy manual", "https://omarchy.org/manual")
                            .title("The Omarchy manual"),
                    )
                    .child(caption("Opens in the system browser.")),
            ),
        )
        .child(
            section("In the app").child(
                row()
                    .child(
                        c::link("Release notes", "/releases")
                            .open_with(LinkOpenWith::App)
                            .on_open(move |href: &SharedString| opened.set(href.clone())),
                    )
                    .child(caption(move || format!("Opened: {}", opened.get()))),
            ),
        )
        .child(
            section("Disabled").child(
                row()
                    .child(c::link("Unavailable link", "https://omarchy.org").disabled(true))
                    .child(caption("No hover, press or focus; Tab skips it.")),
            ),
        )
        .into_element()
}
