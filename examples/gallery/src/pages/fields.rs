//! The fields pages: text, numbers, codes, choices, days and colors. Every
//! demo shows the value the page holds under it.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, Date, DateRange, Hsla, SelectItem, tokens};

use crate::registry::Page;

/// The width the demos take.
const WIDTH: f32 = 280.0;

/// A page: its intro line and its demos, stacked.
fn page(intro: &'static str, demos: Vec<AnyElement>) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .items_start()
        .child(
            text(intro)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::secondary()),
        )
        .children(demos)
        .into_element()
}

/// A demo: the control in a [`om::field`], and a readout of its value.
fn demo(field: om::FieldElement, readout: impl IntoReactive<SharedString>) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_1))
        .min_w(px(WIDTH))
        .child(field)
        .child(
            text(readout)
                .text_size(px(tokens::TEXT_SM))
                .font_family(move || tokens::font_mono())
                .text_color(move || tokens::secondary()),
        )
        .into_element()
}

pub fn input() -> AnyElement {
    let name = signal(String::new());
    let submitted = signal(String::new());
    let secret = signal(String::new());
    page(
        "A single-line field: accent border while it edits, secondary placeholder.",
        vec![
            demo(
                om::field("Workspace name").description("Return submits it.").child(
                    om::input()
                        .value(move || name.get())
                        .placeholder("Workspace name")
                        .on_change(move |t: &String| name.set(t.clone()))
                        .on_submit(move |t: &String| submitted.set(t.clone())),
                ),
                move || {
                    SharedString::from(format!(
                        "value: {:?}  submitted: {:?}",
                        name.get(),
                        submitted.get()
                    ))
                },
            ),
            demo(
                om::field("Passphrase")
                    .required(true)
                    .error(move || {
                        let short = secret.with(|s| !s.is_empty() && s.chars().count() < 8);
                        short.then(|| SharedString::from("At least 8 characters."))
                    })
                    .child(
                        om::input()
                            .masked(true)
                            .invalid(move || secret.with(|s| !s.is_empty() && s.chars().count() < 8))
                            .placeholder("Passphrase")
                            .on_change(move |t: &String| secret.set(t.clone())),
                    ),
                move || SharedString::from(format!("{} characters", secret.with(|s| s.chars().count()))),
            ),
            demo(
                om::field("Sizes").child(
                    view()
                        .gap(px(tokens::SPACE_2))
                        .child(om::input().small().placeholder("Small"))
                        .child(om::input().placeholder("Medium"))
                        .child(om::input().large().placeholder("Large")),
                ),
                "small, medium, large",
            ),
            demo(
                om::field("Config path").child(om::input().value("~/.config/omarchy").disabled(true)),
                "disabled",
            ),
        ],
    )
}

pub fn textarea() -> AnyElement {
    let notes = signal(String::new());
    page(
        "Several lines; it grows with its text. Return adds a line.",
        vec![demo(
            om::field("Notes").child(
                om::textarea()
                    .value(move || notes.get())
                    .rows(3)
                    .placeholder("Write a note")
                    .on_change(move |t: &String| notes.set(t.clone())),
            ),
            move || {
                notes.with(|n| {
                    SharedString::from(format!(
                        "{} lines, {} characters",
                        n.lines().count().max(1),
                        n.chars().count()
                    ))
                })
            },
        )],
    )
}

fn number_text(value: Option<f64>) -> SharedString {
    value.map_or_else(|| "value: none".into(), |v| format!("value: {v}").into())
}

pub fn number_input() -> AnyElement {
    let gaps = signal(Some(4.0_f64));
    let opacity = signal(Some(0.9_f64));
    page(
        "Type a number, or step it: − / +, Up / Down, Page Up / Page Down for ten steps.",
        vec![
            demo(
                om::field("Window gaps").description("0 to 20 pixels.").child(
                    om::number_input()
                        .value(move || gaps.get())
                        .min(0.0)
                        .max(20.0)
                        .suffix("px")
                        .on_change(move |v: &Option<f64>| gaps.set(*v)),
                ),
                move || number_text(gaps.get()),
            ),
            demo(
                om::field("Opacity").child(
                    om::number_input()
                        .value(move || opacity.get())
                        .min(0.0)
                        .max(1.0)
                        .step(0.05)
                        .on_change(move |v: &Option<f64>| opacity.set(*v)),
                ),
                move || number_text(opacity.get()),
            ),
            demo(
                om::field("Monitors").child(om::number_input().value(2.0).disabled(true)),
                "disabled",
            ),
        ],
    )
}

pub fn otp_input() -> AnyElement {
    let code = signal(String::new());
    let complete = signal(String::new());
    let pin = signal(String::new());
    page(
        "Digits only; paste or autofill fills every cell. Backspace removes the last digit.",
        vec![
            demo(
                om::field("Verification code").child(
                    om::otp_input(6)
                        .groups(2)
                        .value(move || code.get())
                        .accessibility_label("Verification code, 6 digits")
                        .on_change(move |v: &String| {
                            code.set(v.clone());
                            complete.set(String::new());
                        })
                        .on_complete(move |v: &String| complete.set(v.clone())),
                ),
                move || {
                    SharedString::from(if complete.with(String::is_empty) {
                        format!("code: {:?}", code.get())
                    } else {
                        format!("complete: {}", complete.get())
                    })
                },
            ),
            demo(
                om::field("PIN")
                    .error(move || (pin.get() == "0000").then(|| SharedString::from("Too easy to guess.")))
                    .child(
                        om::otp_input(4)
                            .masked(true)
                            .value(move || pin.get())
                            .invalid(move || pin.get() == "0000")
                            .accessibility_label("PIN, 4 digits")
                            .on_change(move |v: &String| pin.set(v.clone())),
                    ),
                move || SharedString::from(format!("{} of 4 digits", pin.with(String::len))),
            ),
        ],
    )
}

fn layouts() -> Vec<SelectItem> {
    vec![
        SelectItem::new("dwindle", "Dwindle").group("Tiling"),
        SelectItem::new("master", "Master").group("Tiling"),
        SelectItem::new("scrolling", "Scrolling").group("Tiling"),
        SelectItem::new("floating", "Floating").group("Other").disabled(true),
        SelectItem::new("monocle", "Monocle").group("Other"),
    ]
}

fn themes() -> Vec<SelectItem> {
    [
        "Tokyo Night",
        "Catppuccin",
        "Everforest",
        "Gruvbox",
        "Kanagawa",
        "Nord",
        "Rose Pine",
        "Matte Black",
        "Osaka Jade",
        "Ristretto",
        "Flexoki Light",
    ]
    .into_iter()
    .map(|name| SelectItem::new(name.to_lowercase().replace(' ', "-"), name))
    .collect()
}

fn chosen_text(value: Option<String>) -> SharedString {
    format!("value: {}", value.as_deref().unwrap_or("none")).into()
}

pub fn select() -> AnyElement {
    let layout = signal(Some("dwindle".to_owned()));
    let theme = signal(None::<String>);
    page(
        "Return, Space or Down opens it; Up / Down / Home / End move, a letter jumps, Escape closes.",
        vec![
            demo(
                om::field("Layout").child(
                    om::select(layouts())
                        .value(move || layout.get())
                        .accessibility_label("Layout")
                        .on_change(move |v: &String| layout.set(Some(v.clone()))),
                ),
                move || chosen_text(layout.get()),
            ),
            demo(
                om::field("Theme").description("Type to search.").child(
                    om::select(themes())
                        .value(move || theme.get())
                        .placeholder("Choose a theme")
                        .searchable(true)
                        .accessibility_label("Theme")
                        .on_change(move |v: &String| theme.set(Some(v.clone()))),
                ),
                move || chosen_text(theme.get()),
            ),
            demo(
                om::field("Locked layout").child(
                    om::select(layouts())
                        .value("master")
                        .disabled(true)
                        .small()
                        .accessibility_label("Locked layout"),
                ),
                "disabled",
            ),
        ],
    )
}

pub fn combobox() -> AnyElement {
    let theme = signal(Vec::<String>::new());
    let pinned = signal(vec!["tokyo-night".to_owned()]);
    page(
        "Type to filter; Up / Down move, Return chooses, Escape closes.",
        vec![
            demo(
                om::field("Theme").child(
                    om::combobox(themes())
                        .value(move || theme.get())
                        .placeholder("Search themes")
                        .accessibility_label("Theme")
                        .on_change(move |v: &Vec<String>| theme.set(v.clone())),
                ),
                move || {
                    SharedString::from(format!(
                        "value: {}",
                        theme.with(|t| t.first().cloned().unwrap_or_else(|| "none".into()))
                    ))
                },
            ),
            demo(
                om::field("Pinned themes").description("Several: a choice toggles one.").child(
                    om::combobox(themes())
                        .value(move || pinned.get())
                        .multiple(true)
                        .placeholder("Search themes")
                        .accessibility_label("Pinned themes")
                        .on_change(move |v: &Vec<String>| pinned.set(v.clone())),
                ),
                move || SharedString::from(format!("pinned: {}", pinned.get().join(", "))),
            ),
        ],
    )
}

fn day_text(what: &str, day: Option<Date>) -> SharedString {
    day.map_or_else(|| format!("{what}: none"), |d| format!("{what}: {d}"))
        .into()
}

fn range_text(span: Option<DateRange>) -> SharedString {
    span.map_or_else(
        || "range: none".into(),
        |r| format!("range: {} to {}", r.start, r.end),
    )
    .into()
}

pub fn calendar() -> AnyElement {
    let picked = signal(None::<Date>);
    let span = signal(None::<DateRange>);
    view()
        .flex_row()
        .flex_wrap()
        .gap(px(tokens::SPACE_6))
        .child(demo(
            om::field("Calendar").child(
                om::calendar()
                    .value(move || picked.get())
                    .on_change(move |d: &Date| picked.set(Some(*d))),
            ),
            move || day_text("picked", picked.get()),
        ))
        .child(demo(
            om::field("Range").description("Up to today.").child(
                om::calendar()
                    .range(true)
                    .range_value(move || span.get())
                    .max(Date::today())
                    .on_range_change(move |r: &DateRange| span.set(Some(*r))),
            ),
            move || range_text(span.get()),
        ))
        .into_element()
}

pub fn date_picker() -> AnyElement {
    let due = signal(None::<Date>);
    let trip = signal(None::<DateRange>);
    page(
        "Return or Space opens the calendar; a pick or Escape closes it.",
        vec![
            demo(
                om::field("Due date").child(
                    om::date_picker()
                        .value(move || due.get())
                        .accessibility_label("Due date")
                        .on_change(move |d: &Date| due.set(Some(*d))),
                ),
                move || day_text("due", due.get()),
            ),
            demo(
                om::field("Trip dates").child(
                    om::date_picker()
                        .range(true)
                        .range_value(move || trip.get())
                        .min(Date::today())
                        .accessibility_label("Trip dates")
                        .on_range_change(move |r: &DateRange| trip.set(Some(*r))),
                ),
                move || range_text(trip.get()),
            ),
            demo(
                om::field("Release").child(
                    om::date_picker()
                        .value(Date::new(2026, 11, 19))
                        .disabled(true)
                        .small()
                        .accessibility_label("Release"),
                ),
                "disabled",
            ),
        ],
    )
}

pub fn color_picker() -> AnyElement {
    let accent = signal(color(tokens::accent()));
    let swatches: Vec<Hsla> = (1..8).map(|i| color(tokens::ansi(i))).collect();
    page(
        "Drag the area or the sliders, type a CSS color, or press a swatch.",
        vec![demo(
            om::field("Accent").child(
                om::color_picker()
                    .value(move || accent.get())
                    .alpha(true)
                    .swatches(swatches)
                    .accessibility_label("Accent")
                    .on_change(move |c: &Hsla| accent.set(*c)),
            ),
            move || SharedString::from(format!("color: {}", accent.get().to_hex())),
        )],
    )
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Forms", id: "input", title: "Input", make: input },
    Page { group: "Forms", id: "textarea", title: "Textarea", make: textarea },
    Page { group: "Forms", id: "number_input", title: "Number input", make: number_input },
    Page { group: "Forms", id: "otp_input", title: "Otp input", make: otp_input },
    Page { group: "Forms", id: "select", title: "Select", make: select },
    Page { group: "Forms", id: "combobox", title: "Combobox", make: combobox },
    Page { group: "Forms", id: "calendar", title: "Calendar", make: calendar },
    Page { group: "Forms", id: "date_picker", title: "Date picker", make: date_picker },
    Page { group: "Forms", id: "color_picker", title: "Color picker", make: color_picker },
];
