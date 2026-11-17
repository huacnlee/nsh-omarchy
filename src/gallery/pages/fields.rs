//! Gallery pages of the `fields` group. Each `pub fn` is one page.
//!
//! Every demo is controlled: the page owns the value in a signal, the field
//! reports changes and a readout under it shows what the page holds.

use nsh_base::OtpInputCompleteEvent;
use nsh_core::prelude::*;

use crate::components::input::{number_field, text_input, textarea as textarea_field};
use crate::components::otp_input::otp_field;
use crate::components::select::{SelectItem, combobox_field, select_field};
use crate::tokens;

/// The width the demos' fields take.
const FIELD_WIDTH: f32 = 280.0;

/// A labelled demo: a caption, the control, and a readout of its value.
fn demo(
    caption: &'static str,
    control: impl IntoElement,
    readout: impl IntoReactive<SharedString>,
) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_1))
        .w(px(FIELD_WIDTH))
        .child(
            text(caption)
                .text_size(px(tokens::TEXT_SM))
                .font_weight(FontWeight::MEDIUM)
                .text_color(move || tokens::secondary()),
        )
        .child(control)
        .child(
            text(readout)
                .text_size(px(tokens::TEXT_SM))
                .font_family(move || tokens::font_mono())
                .text_color(move || tokens::secondary()),
        )
        .into_element()
}

/// A page's intro line and its demos, stacked.
fn page(intro: &'static str, demos: Vec<AnyElement>) -> AnyElement {
    view()
        .gap(px(tokens::SPACE_6))
        .child(
            text(intro)
                .text_size(px(tokens::TEXT_MD))
                .text_color(move || tokens::secondary()),
        )
        .children(demos)
        .into_element()
}

pub fn input() -> AnyElement {
    let name = signal(String::new());
    let secret = signal(String::new());
    page(
        "A single-line field: accent border while it edits, secondary placeholder.",
        vec![
            demo(
                "Workspace name",
                text_input()
                    .value(move || name.get())
                    .on_change(move |text: &str| name.set(text.to_owned()))
                    .placeholder("Workspace name")
                    .label("Workspace name"),
                move || SharedString::from(format!("value: {:?}", name.get())),
            ),
            demo(
                "Passphrase (masked)",
                text_input()
                    .value(move || secret.get())
                    .on_change(move |text: &str| secret.set(text.to_owned()))
                    .masked(true)
                    .placeholder("Passphrase")
                    .label("Passphrase"),
                move || {
                    SharedString::from(format!("{} characters", secret.with(|s| s.chars().count())))
                },
            ),
            demo(
                "Disabled",
                text_input()
                    .value("~/.config/omarchy")
                    .disabled(true)
                    .label("Config path"),
                "read only",
            ),
        ],
    )
}

pub fn textarea() -> AnyElement {
    let notes = signal(String::new());
    page(
        "Several lines; it grows with its text. Return adds a line.",
        vec![demo(
            "Notes",
            textarea_field()
                .value(move || notes.get())
                .on_change(move |text: &str| notes.set(text.to_owned()))
                .placeholder("Write a note")
                .label("Notes"),
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
                "Window gaps (0 to 20)",
                number_field()
                    .value(move || gaps.get())
                    .on_change(move |v: &Option<f64>| gaps.set(*v))
                    .min(0.0)
                    .max(20.0)
                    .accessibility_label("Window gaps"),
                move || number_text(gaps.get()),
            ),
            demo(
                "Opacity (step 0.05)",
                number_field()
                    .value(move || opacity.get())
                    .on_change(move |v: &Option<f64>| opacity.set(*v))
                    .min(0.0)
                    .max(1.0)
                    .step(0.05)
                    .accessibility_label("Opacity"),
                move || number_text(opacity.get()),
            ),
            demo(
                "Disabled",
                number_field()
                    .value(Some(2.0))
                    .disabled(true)
                    .accessibility_label("Monitors"),
                "read only",
            ),
        ],
    )
}

pub fn otp_input() -> AnyElement {
    let code = signal(SharedString::default());
    let complete = signal(false);
    let pin = signal(SharedString::default());
    page(
        "Digits only; paste or autofill fills every cell. Backspace removes the last digit.",
        vec![
            demo(
                "Verification code",
                otp_field(6)
                    .groups(2)
                    .value(move || Some(code.get()))
                    .on_change(move |v: &SharedString| {
                        code.set(v.clone());
                        complete.set(false);
                    })
                    .on_complete(move |_: &OtpInputCompleteEvent| complete.set(true))
                    .accessibility_label("Verification code, 6 digits"),
                move || {
                    SharedString::from(if complete.get() {
                        format!("complete: {}", code.get())
                    } else {
                        format!("code: {:?}", code.get())
                    })
                },
            ),
            demo(
                "PIN (masked)",
                otp_field(4)
                    .masked(true)
                    .value(move || Some(pin.get()))
                    .on_change(move |v: &SharedString| pin.set(v.clone()))
                    .accessibility_label("PIN, 4 digits"),
                move || SharedString::from(format!("{} of 4 digits", pin.with(|p| p.len()))),
            ),
        ],
    )
}

fn layouts() -> Vec<SelectItem> {
    vec![
        SelectItem::new("dwindle", "Dwindle"),
        SelectItem::new("master", "Master"),
        SelectItem::new("scrolling", "Scrolling"),
        SelectItem::new("floating", "Floating").disabled(true),
        SelectItem::new("monocle", "Monocle"),
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

fn chosen_text(value: Option<SharedString>) -> SharedString {
    format!("value: {}", value.as_deref().unwrap_or("none")).into()
}

pub fn select() -> AnyElement {
    let layout = signal(Some(SharedString::from("dwindle")));
    let theme = signal(None::<SharedString>);
    page(
        "Enter, Space or Down opens it; Up / Down / Home / End move, a letter jumps, Escape closes.",
        vec![
            demo(
                "Layout",
                select_field(layouts(), move || layout.get(), "Choose a layout")
                    .on_change(move |v: &Option<SharedString>| layout.set(v.clone()))
                    .accessibility_label("Layout"),
                move || chosen_text(layout.get()),
            ),
            demo(
                "Theme (searchable)",
                select_field(themes(), move || theme.get(), "Choose a theme")
                    .searchable(true)
                    .search_accessibility_label("Search themes")
                    .on_change(move |v: &Option<SharedString>| theme.set(v.clone()))
                    .accessibility_label("Theme"),
                move || chosen_text(theme.get()),
            ),
            demo(
                "Disabled",
                select_field(layouts(), || Some("master".into()), "Choose a layout")
                    .disabled(true)
                    .accessibility_label("Locked layout"),
                "read only",
            ),
        ],
    )
}

pub fn combobox() -> AnyElement {
    let theme = signal(Vec::<SharedString>::new());
    let query = signal(SharedString::default());
    page(
        "Type to filter; Up / Down move, Return chooses, Escape closes.",
        vec![demo(
            "Theme",
            combobox_field(themes())
                .value(move || Some(theme.get()))
                .on_change(move |v: &Vec<SharedString>| theme.set(v.clone()))
                .on_query_change(move |q: &SharedString| query.set(q.clone()))
                .placeholder("Search themes")
                .accessibility_label("Theme"),
            move || {
                SharedString::from(format!(
                    "value: {}  query: {:?}",
                    theme.with(|t| t
                        .first()
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "none".into())),
                    query.get()
                ))
            },
        )],
    )
}
