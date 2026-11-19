//! The choices pages: checkboxes, switches, radios, sliders, progress.

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, SliderValue, tokens};

use crate::registry::Page;

/// A titled block of demos.
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

/// A line of state the demos report, in the secondary color.
fn status(line: impl Fn() -> String + 'static) -> Text {
    text(line)
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
}

fn on_off(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

fn page() -> View {
    view().gap(px(tokens::SPACE_6)).max_w(px(560.0))
}

pub fn checkbox() -> AnyElement {
    let hidden = signal(false);
    // A "select all" over three items: indeterminate while some are chosen.
    let items = signal([true, false, false]);
    let chosen = computed(move || items.get().iter().filter(|on| **on).count());
    let item = move |index: usize, label: &'static str| {
        om::checkbox(label)
            .checked(move || items.get()[index])
            .on_change(move |next: &bool| items.update(|all| all[index] = *next))
    };
    page()
        .child(section(
            "Default",
            view()
                .child(
                    om::checkbox("Show hidden files")
                        .checked(move || hidden.get())
                        .on_change(move |next: &bool| hidden.set(*next)),
                )
                .child(om::checkbox("Keeps its own value"))
                .child(status(move || {
                    format!("Show hidden files: {}", on_off(hidden.get()))
                })),
        ))
        .child(section(
            "Indeterminate",
            view()
                .child(
                    om::checkbox("All file types")
                        .checked(move || chosen.get() == 3)
                        .indeterminate(move || (1..3).contains(&chosen.get()))
                        .on_change(move |next: &bool| items.set([*next; 3])),
                )
                .child(
                    view()
                        .pl(px(tokens::SPACE_6))
                        .child(item(0, "Images"))
                        .child(item(1, "Documents"))
                        .child(item(2, "Archives")),
                )
                .child(status(move || format!("{} of 3 chosen", chosen.get()))),
        ))
        .child(section(
            "Disabled",
            view()
                .child(om::checkbox("Disabled").disabled(true))
                .child(om::checkbox("Disabled, checked").checked(true).disabled(true))
                .child(
                    om::checkbox("Disabled, indeterminate")
                        .indeterminate(true)
                        .disabled(true),
                ),
        ))
        .into_element()
}

pub fn switch() -> AnyElement {
    let wifi = signal(false);
    let bluetooth = signal(true);
    page()
        .child(section(
            "Default",
            view()
                .child(
                    om::switch("Wi-Fi")
                        .checked(move || wifi.get())
                        .on_change(move |next: &bool| wifi.set(*next)),
                )
                .child(
                    om::switch("Bluetooth")
                        .checked(move || bluetooth.get())
                        .on_change(move |next: &bool| bluetooth.set(*next)),
                )
                .child(status(move || {
                    format!(
                        "Wi-Fi {}, Bluetooth {}",
                        on_off(wifi.get()),
                        on_off(bluetooth.get())
                    )
                })),
        ))
        .child(section(
            "Disabled",
            view()
                .child(om::switch("Airplane mode").disabled(true))
                .child(om::switch("Location").checked(true).disabled(true)),
        ))
        .into_element()
}

pub fn radio() -> AnyElement {
    let mode = signal(SharedString::from("dark"));
    let density = signal(SharedString::from("compact"));
    page()
        .child(section(
            "Column",
            view()
                .child(
                    om::radio_group()
                        .accessibility_label("Appearance")
                        .value(move || mode.get())
                        .on_change(move |next: &SharedString| mode.set(next.clone()))
                        .child(om::radio("light", "Light"))
                        .child(om::radio("dark", "Dark"))
                        .child(om::radio("system", "System"))
                        .child(om::radio("contrast", "High contrast").disabled(true)),
                )
                .child(status(move || format!("Appearance: {}", mode.get()))),
        ))
        .child(section(
            "Row",
            view()
                .child(
                    om::radio_group()
                        .accessibility_label("Density")
                        .horizontal()
                        .value(move || density.get())
                        .on_change(move |next: &SharedString| density.set(next.clone()))
                        .child(om::radio("compact", "Compact"))
                        .child(om::radio("cozy", "Cozy"))
                        .child(om::radio("roomy", "Roomy")),
                )
                .child(status(move || format!("Density: {}", density.get()))),
        ))
        .into_element()
}

pub fn slider() -> AnyElement {
    let volume = signal(SliderValue::Single(40.0));
    let price = signal(SliderValue::Range(20.0, 80.0));
    page()
        .child(section(
            "Single value",
            view()
                .child(
                    om::slider()
                        .accessibility_label("Volume")
                        .show_value(true)
                        .value(move || volume.get())
                        .on_change(move |next: &SliderValue| volume.set(*next)),
                )
                .child(status(move || format!("Volume {}", volume.get().end()))),
        ))
        .child(section(
            "Step 10, keeps its own value",
            om::slider()
                .accessibility_label("Brightness")
                .step(10.0)
                .show_value(true),
        ))
        .child(section(
            "Range",
            om::slider()
                .accessibility_label("Price range")
                .show_value(true)
                .value(move || price.get())
                .on_change(move |next: &SliderValue| price.set(*next)),
        ))
        .child(section(
            "Disabled",
            om::slider()
                .accessibility_label("Locked")
                .value(SliderValue::Single(30.0))
                .disabled(true),
        ))
        .into_element()
}

pub fn progress() -> AnyElement {
    let done = signal(SliderValue::Single(35.0));
    let percent = move || done.get().end();
    page()
        .child(section(
            "Determinate",
            view()
                .gap(px(tokens::SPACE_2))
                .child(
                    om::progress()
                        .label("Downloading…")
                        .show_value(true)
                        .value(percent),
                )
                .child(
                    om::slider()
                        .accessibility_label("Downloaded")
                        .value(move || done.get())
                        .on_change(move |next: &SliderValue| done.set(*next)),
                ),
        ))
        .child(section(
            "Indeterminate",
            om::progress().label("Installing").indeterminate(true),
        ))
        .child(section(
            "Tones",
            view()
                .gap(px(tokens::SPACE_3))
                .child(om::progress().label("Backup").value(80.0).success().show_value(true))
                .child(om::progress().label("Disk").value(60.0).warning().show_value(true))
                .child(om::progress().label("Battery").value(12.0).danger().show_value(true)),
        ))
        .into_element()
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Forms", id: "checkbox", title: "Checkbox", make: checkbox },
    Page { group: "Forms", id: "switch", title: "Switch", make: switch },
    Page { group: "Forms", id: "radio", title: "Radio", make: radio },
    Page { group: "Forms", id: "slider", title: "Slider", make: slider },
    Page { group: "Forms", id: "progress", title: "Progress", make: progress },
];
