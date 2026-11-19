//! Gallery pages of the `choices` group. Each `pub fn` is one page.

use nsh_base::SliderValue;
use nsh_core::prelude::*;

use crate::components as ui;
use crate::tokens;

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
        ui::checkbox(label)
            .checked(move || items.get()[index])
            .on_change(move |next: &bool| items.update(|all| all[index] = *next))
    };
    page()
        .child(section(
            "Default",
            view()
                .child(
                    ui::checkbox("Show hidden files")
                        .checked(hidden)
                        .on_change(move |next: &bool| hidden.set(*next)),
                )
                .child(status(move || {
                    format!("Show hidden files: {}", on_off(hidden.get()))
                })),
        ))
        .child(section(
            "Indeterminate",
            view()
                .child(
                    ui::checkbox("All file types")
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
                .child(ui::checkbox("Disabled").disabled(true))
                .child(
                    ui::checkbox("Disabled, checked")
                        .checked(true)
                        .disabled(true),
                )
                .child(
                    ui::checkbox("Disabled, indeterminate")
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
                    ui::switch("Wi-Fi")
                        .checked(wifi)
                        .on_change(move |next: &bool| wifi.set(*next)),
                )
                .child(
                    ui::switch("Bluetooth")
                        .checked(bluetooth)
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
                .child(ui::switch("Airplane mode").disabled(true))
                .child(ui::switch("Location").checked(true).disabled(true)),
        ))
        .into_element()
}

pub fn radio() -> AnyElement {
    const MODES: [&str; 4] = ["Light", "Dark", "System", "High contrast"];
    const DENSITIES: [&str; 3] = ["Compact", "Cozy", "Roomy"];
    let mode = signal(1_usize);
    let density = signal(0_usize);
    let option = move |chosen: Signal<usize>, index: usize, label: &'static str| {
        ui::radio(label)
            .checked(move || chosen.get() == index)
            .on_change(move |_: &bool| chosen.set(index))
    };
    page()
        .child(section(
            "Column",
            view()
                .child(
                    ui::radio_group()
                        .child(option(mode, 0, MODES[0]))
                        .child(option(mode, 1, MODES[1]))
                        .child(option(mode, 2, MODES[2]))
                        .child(option(mode, 3, MODES[3]).disabled(true)),
                )
                .child(status(move || format!("Appearance: {}", MODES[mode.get()]))),
        ))
        .child(section(
            "Row",
            view()
                .child(
                    ui::radio_group()
                        .flex_row()
                        .gap(px(tokens::SPACE_3))
                        .children(
                            DENSITIES
                                .iter()
                                .enumerate()
                                .map(|(i, label)| option(density, i, label)),
                        ),
                )
                .child(status(move || {
                    format!("Density: {}", DENSITIES[density.get()])
                })),
        ))
        .into_element()
}

/// The value of a slider as a short label.
fn shown(value: SliderValue) -> String {
    match value {
        SliderValue::Single(v) => format!("{v:.0}"),
        SliderValue::Range(a, b) => format!("{a:.0}–{b:.0}"),
    }
}

/// The value column after a slider's rail.
fn value_label(value: Signal<SliderValue>) -> Text {
    text(move || shown(value.get()))
        .w(px(56.0))
        .text_right()
        .tabular_nums()
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
}

pub fn slider() -> AnyElement {
    let volume = signal(SliderValue::Single(40.0));
    let brightness = signal(SliderValue::Single(70.0));
    let price = signal(SliderValue::Range(20.0, 80.0));
    page()
        .child(section(
            "Single value",
            view()
                .child(
                    ui::slider()
                        .accessibility_label("Volume")
                        .value(move || Some(volume.get()))
                        .on_change(move |next: &SliderValue| volume.set(*next))
                        .child(value_label(volume)),
                )
                .child(status(move || format!("Volume {}", shown(volume.get())))),
        ))
        .child(section(
            "Step 10",
            ui::slider()
                .accessibility_label("Brightness")
                .step(10.0)
                .value(move || Some(brightness.get()))
                .on_change(move |next: &SliderValue| brightness.set(*next))
                .child(value_label(brightness)),
        ))
        .child(section(
            "Range",
            ui::slider()
                .accessibility_label("Highest price")
                .start_accessibility_label("Lowest price")
                .value(move || Some(price.get()))
                .on_change(move |next: &SliderValue| price.set(*next))
                .child(value_label(price)),
        ))
        .child(section(
            "Disabled",
            ui::slider()
                .accessibility_label("Locked")
                .value(SliderValue::Single(30.0))
                .disabled(true),
        ))
        .into_element()
}

pub fn progress() -> AnyElement {
    let done = signal(SliderValue::Single(35.0));
    let percent = move || done.get().end() as f32;
    page()
        .child(section(
            "Determinate",
            view()
                .gap(px(tokens::SPACE_2))
                .child(
                    ui::progress()
                        .accessibility_label("Download")
                        .value(percent)
                        .child(text(move || format!("Downloading… {:.0}%", percent()))),
                )
                .child(
                    ui::slider()
                        .accessibility_label("Downloaded")
                        .value(move || Some(done.get()))
                        .on_change(move |next: &SliderValue| done.set(*next)),
                ),
        ))
        .child(section(
            "Values",
            view()
                .gap(px(tokens::SPACE_3))
                .child(
                    ui::progress()
                        .accessibility_label("Empty")
                        .value(0.0)
                        .child(text("0%")),
                )
                .child(
                    ui::progress()
                        .accessibility_label("Done")
                        .value(100.0)
                        .child(text("100%")),
                ),
        ))
        .child(section(
            "Tones",
            view()
                .gap(px(tokens::SPACE_3))
                .child(
                    ui::progress_tone(tokens::success)
                        .accessibility_label("Backup")
                        .value(80.0)
                        .child(text("Backup 80%")),
                )
                .child(
                    ui::progress_tone(tokens::warning)
                        .accessibility_label("Disk")
                        .value(60.0)
                        .child(text("Disk 60%")),
                )
                .child(
                    ui::progress_tone(tokens::danger)
                        .accessibility_label("Battery")
                        .value(12.0)
                        .child(text("Battery 12%")),
                ),
        ))
        .into_element()
}
