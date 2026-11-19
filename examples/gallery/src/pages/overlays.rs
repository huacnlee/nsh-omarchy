//! The overlays pages: dialogs, sheets, popovers, tooltips, hover cards
//! and toasts.

use std::time::Duration;

use nsh_core::prelude::*;
use nsh_omarchy::{self as om, tokens};

use crate::registry::Page;

/// A short explanation under the page title.
fn lead(copy: &'static str) -> impl IntoElement {
    text(copy)
        .text_size(px(tokens::TEXT_MD))
        .text_color(move || tokens::secondary())
}

/// A row of demo controls.
fn row() -> View {
    view()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(tokens::SPACE_2))
}

/// The last outcome of a demo, as a caption.
fn outcome(last: Signal<SharedString>) -> impl IntoElement {
    text(move || format!("Last: {}", last.get()))
        .text_size(px(tokens::TEXT_SM))
        .text_color(move || tokens::secondary())
}

fn page() -> View {
    view().gap(px(tokens::SPACE_4)).items_start()
}

pub fn dialog() -> AnyElement {
    let open = signal(false);
    let last = signal(SharedString::from("none"));
    page()
        .child(lead(
            "A modal card for a short task. Escape, a backdrop press or Cancel dismiss it.",
        ))
        .child(row().child(om::button("Rename workspace").on_press(move || open.set(true))))
        .child(outcome(last))
        .child(
            om::dialog("Rename workspace")
                .description("The name shows in the bar and the workspace switcher.")
                .open(move || open.get())
                .on_open_change(move |o: &bool| {
                    open.set(*o);
                    if !*o {
                        last.set("dismissed".into());
                    }
                })
                .footer(
                    view()
                        .flex_row()
                        .gap(px(tokens::SPACE_2))
                        .child(om::button("Cancel").on_press(move || {
                            open.set(false);
                            last.set("dismissed".into());
                        }))
                        .child(om::button("Rename").primary().on_press(move || {
                            open.set(false);
                            last.set("renamed".into());
                        })),
                )
                .child(
                    view()
                        .px(px(tokens::SPACE_2))
                        .py(px(tokens::SPACE_1))
                        .border(px(tokens::BORDER))
                        .border_color(move || tokens::control_border())
                        .bg(move || tokens::inset())
                        .child(text("3: code").font_family(tokens::font_mono())),
                ),
        )
        .into_element()
}

pub fn alert_dialog() -> AnyElement {
    let open = signal(false);
    let last = signal(SharedString::from("none"));
    page()
        .child(lead(
            "A decision. Cancel takes the first focus and a backdrop press does nothing; \
             Escape cancels. \"Native confirm\" asks the platform's own alert.",
        ))
        .child(
            row()
                .child(om::button("Delete workspace").danger().on_press(move || open.set(true)))
                .child(om::button("Native confirm").on_press(move || {
                    spawn(async move {
                        let yes = om::confirm(
                            om::ConfirmOptions::new("Delete workspace 3?")
                                .description("Its windows move to workspace 1.")
                                .action_label("Delete")
                                .danger(),
                        )
                        .await;
                        last.set(if yes { "native: deleted" } else { "native: kept" }.into());
                    });
                })),
        )
        .child(outcome(last))
        .child(
            om::alert_dialog("Delete workspace 3?", "Delete")
                .description("Its 2 windows move to workspace 1. This cannot be undone.")
                .danger()
                .open(move || open.get())
                .on_open_change(move |o: &bool| {
                    open.set(*o);
                    // Before `on_action`: a confirm ends "deleted".
                    if !*o {
                        last.set("kept".into());
                    }
                })
                .on_action(move || last.set("deleted".into())),
        )
        .into_element()
}

pub fn sheet() -> AnyElement {
    let right = signal(false);
    let bottom = signal(false);
    let last = signal(SharedString::from("none"));
    let setting = |name: &'static str, value: &'static str| {
        view()
            .flex_row()
            .justify_between()
            .py(px(tokens::SPACE_2))
            .border_b(px(tokens::BORDER))
            .border_color(move || tokens::divider())
            .child(text(name))
            .child(text(value).text_color(move || tokens::secondary()))
    };
    page()
        .child(lead(
            "An edge panel for details and settings. It slides in from its edge; the close \
             button, Escape and a backdrop press dismiss it.",
        ))
        .child(
            row()
                .child(om::button("Display settings").on_press(move || right.set(true)))
                .child(om::button("Bottom sheet").on_press(move || bottom.set(true))),
        )
        .child(outcome(last))
        .child(
            om::sheet("Display")
                .description("Applies to every monitor.")
                .open(move || right.get())
                .on_open_change(move |o: &bool| {
                    right.set(*o);
                    if !*o {
                        last.set("sheet dismissed".into());
                    }
                })
                .footer(om::button("Apply").primary().on_press(move || {
                    right.set(false);
                    last.set("applied".into());
                }))
                .child(setting("Scale", "1.25×"))
                .child(setting("Refresh rate", "144 Hz"))
                .child(setting("Night light", "Off")),
        )
        .child(
            om::sheet("Clipboard")
                .placement(Placement::Bottom)
                .open(move || bottom.get())
                .on_open_change(move |o: &bool| {
                    bottom.set(*o);
                    if !*o {
                        last.set("bottom dismissed".into());
                    }
                })
                .child(text("3 items").text_color(move || tokens::secondary())),
        )
        .into_element()
}

pub fn popover() -> AnyElement {
    page()
        .child(lead(
            "Anchored, non-modal content. Its trigger toggles it; a press outside and Escape \
             close it and the focus returns to the trigger.",
        ))
        .child(
            row()
                .child(
                    om::popover()
                        .trigger(om::button("Workspace details"))
                        .title("Workspace 3")
                        .child(text("2 windows · code, terminal"))
                        .child(
                            text("Super + 3 to switch")
                                .text_size(px(tokens::TEXT_SM))
                                .text_color(move || tokens::secondary()),
                        ),
                )
                .child(
                    om::popover()
                        .placement(Placement::Right)
                        .trigger(om::button("Battery"))
                        .title("Battery 82%")
                        .child(text("3 h 40 min left")),
                ),
        )
        .into_element()
}

pub fn tooltip() -> AnyElement {
    page()
        .child(lead(
            "A short hint after the pointer rests 400 ms, or at once when the keyboard \
             focuses its control. Escape hides it.",
        ))
        .child(
            row()
                .child(om::button("Save").tooltip(om::tooltip("Save").shortcut("mod-s")))
                .child(
                    om::icon_button(om::IconName::Lock, "Lock")
                        .tooltip(om::tooltip("Lock the screen").shortcut("super-ctrl-l")),
                )
                .child(om::button("Help").ghost().tooltip(om::tooltip("Keyboard shortcuts"))),
        )
        .into_element()
}

pub fn hover_card() -> AnyElement {
    page()
        .child(lead(
            "A preview while the pointer rests on its trigger. Supplementary: what a keyboard \
             user needs stays reachable outside it.",
        ))
        .child(
            row().child(text("Theme by")).child(
                om::hover_card()
                    .trigger(om::button("@tokyo-night").ghost())
                    .child(
                        text("Tokyo Night")
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(move || tokens::bright()),
                    )
                    .child(text("A clean dark theme that celebrates the lights of Tokyo."))
                    .child(
                        text("Built in · 16 colors")
                            .text_size(px(tokens::TEXT_SM))
                            .text_color(move || tokens::secondary()),
                    ),
            ),
        )
        .into_element()
}

pub fn toast() -> AnyElement {
    let restored = signal(0_u32);
    let last = signal(String::new());
    page()
        .child(lead(
            "Notices at the bottom right. They leave after 6 s; the pointer on the stack \
             pauses them. Errors stay until dismissed. Escape dismisses the newest.",
        ))
        .child(
            row()
                .child(om::button("Notify").on_press(move || {
                    om::toast(
                        om::Toast::new("Screenshot saved").description("~/Pictures/screenshot.png"),
                    );
                }))
                .child(om::button("Success").on_press(move || {
                    om::toast(om::Toast::new("Update installed").success());
                }))
                .child(om::button("Warning").on_press(move || {
                    om::toast(om::Toast::new("Battery low").description("15% left.").warning());
                }))
                .child(om::button("Error").danger().on_press(move || {
                    om::toast(
                        om::Toast::new("Wi-Fi disconnected")
                            .description("Retrying in 30 s.")
                            .error(),
                    );
                }))
                .child(om::button("Close window").on_press(move || {
                    let id = om::toast(
                        om::Toast::new("Window closed")
                            .timeout(Duration::from_secs(8))
                            .action("Undo", move || restored.update(|n| *n += 1)),
                    );
                    last.set(id);
                }))
                .child(
                    om::button("Dismiss last")
                        .ghost()
                        .on_press(move || om::dismiss_toast(&last.get_untracked())),
                ),
        )
        .child(
            text(move || format!("Restored: {}", restored.get()))
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
        // The gallery's root has no toaster: this page places its own.
        .child(om::toaster())
        .into_element()
}

/// This area's pages, in sidebar order.
pub const PAGES: &[Page] = &[
    Page { group: "Overlays", id: "dialog", title: "Dialog", make: dialog },
    Page { group: "Overlays", id: "alert_dialog", title: "Alert dialog", make: alert_dialog },
    Page { group: "Overlays", id: "sheet", title: "Sheet", make: sheet },
    Page { group: "Overlays", id: "popover", title: "Popover", make: popover },
    Page { group: "Overlays", id: "tooltip", title: "Tooltip", make: tooltip },
    Page { group: "Overlays", id: "hover_card", title: "Hover card", make: hover_card },
    Page { group: "Overlays", id: "toast", title: "Toast", make: toast },
];
