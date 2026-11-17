//! Gallery pages of the `overlays` group. Each `pub fn` is one page.

use std::time::Duration;

use nsh_core::prelude::*;

use crate::components as c;
use crate::components::OverlayTone;
use crate::tokens;

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
    let trigger = element_ref();
    page()
        .child(lead(
            "A modal card for a short task. Escape, a backdrop press or Cancel dismiss it; \
             the focus goes back to the button that opened it.",
        ))
        .child(
            row().child(
                c::overlay_action_button("Rename workspace", OverlayTone::Neutral, move || {
                    open.set(true)
                })
                .element_ref(&trigger),
            ),
        )
        .child(outcome(last))
        .child(
            c::dialog(open, "Rename workspace")
                .description("The name shows in the bar and the workspace switcher.")
                .body(|| {
                    view()
                        .px(px(tokens::SPACE_2))
                        .py(px(tokens::SPACE_1))
                        .border(px(tokens::BORDER))
                        .border_color(move || tokens::control_border())
                        .bg(move || tokens::inset())
                        .child(text("3: code").font_family(tokens::font_mono()))
                })
                .on_dismiss(move || last.set("dismissed".into()))
                .cancel("Cancel")
                .action("Rename", OverlayTone::Primary, move || {
                    last.set("renamed".into())
                })
                .return_focus(trigger),
        )
        .into_element()
}

pub fn alert_dialog() -> AnyElement {
    let open = signal(false);
    let last = signal(SharedString::from("none"));
    let trigger = element_ref();
    page()
        .child(lead(
            "A destructive confirm. Cancel takes the first focus and a backdrop press does \
             nothing; Escape cancels. \"Native confirm\" asks the platform's own alert.",
        ))
        .child(
            row()
                .child(
                    c::overlay_action_button("Delete workspace", OverlayTone::Danger, move || {
                        open.set(true)
                    })
                    .element_ref(&trigger),
                )
                .child(c::overlay_action_button(
                    "Native confirm",
                    OverlayTone::Neutral,
                    move || {
                        spawn(async move {
                            let yes = c::confirm_native(
                                "Delete workspace 3?",
                                "Its windows move to workspace 1.",
                                "Delete",
                            )
                            .await;
                            last.set(
                                if yes {
                                    "native: deleted"
                                } else {
                                    "native: kept"
                                }
                                .into(),
                            );
                        });
                    },
                )),
        )
        .child(outcome(last))
        .child(
            c::alert_dialog(open, "Delete workspace 3?")
                .description("Its 2 windows move to workspace 1. This cannot be undone.")
                .on_dismiss(move || last.set("kept".into()))
                .cancel("Cancel")
                .action("Delete", OverlayTone::Danger, move || {
                    last.set("deleted".into())
                })
                .return_focus(trigger),
        )
        .into_element()
}

pub fn sheet() -> AnyElement {
    let right = signal(false);
    let bottom = signal(false);
    let last = signal(SharedString::from("none"));
    let right_trigger = element_ref();
    let bottom_trigger = element_ref();
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
                .child(
                    c::overlay_action_button("Display settings", OverlayTone::Neutral, move || {
                        right.set(true)
                    })
                    .element_ref(&right_trigger),
                )
                .child(
                    c::overlay_action_button("Bottom sheet", OverlayTone::Neutral, move || {
                        bottom.set(true)
                    })
                    .element_ref(&bottom_trigger),
                ),
        )
        .child(outcome(last))
        .child(
            c::sheet(right, "Display")
                .description("Applies to every monitor.")
                .body(move || {
                    view()
                        .child(setting("Scale", "1.25×"))
                        .child(setting("Refresh rate", "144 Hz"))
                        .child(setting("Night light", "Off"))
                })
                .on_dismiss(move || last.set("sheet dismissed".into()))
                .action("Apply", OverlayTone::Primary, move || {
                    last.set("applied".into())
                })
                .return_focus(right_trigger),
        )
        .child(
            c::sheet(bottom, "Clipboard")
                .side(Placement::Bottom)
                .size(220.0)
                .body(|| text("3 items").text_color(move || tokens::secondary()))
                .on_dismiss(move || last.set("bottom dismissed".into()))
                .return_focus(bottom_trigger),
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
                    c::popover()
                        .trigger(c::overlay_action_button(
                            "Workspace details",
                            OverlayTone::Neutral,
                            || {},
                        ))
                        .child(c::popover_title("Workspace 3"))
                        .child(text("2 windows · code, terminal"))
                        .child(
                            text("Super + 3 to switch")
                                .text_size(px(tokens::TEXT_SM))
                                .text_color(move || tokens::secondary()),
                        ),
                )
                .child(
                    c::popover()
                        .placement(Placement::Right)
                        .trigger(c::overlay_action_button(
                            "Battery",
                            OverlayTone::Neutral,
                            || {},
                        ))
                        .child(c::popover_title("Battery 82%"))
                        .child(text("3 h 40 min left")),
                ),
        )
        .into_element()
}

pub fn tooltip() -> AnyElement {
    page()
        .child(lead(
            "A short hint after the pointer rests 400 ms, or at once when the keyboard \
             focuses its trigger. Escape hides it.",
        ))
        .child(
            row()
                .child(c::with_tooltip(
                    c::overlay_action_button("Save", OverlayTone::Neutral, || {}),
                    "Save  Ctrl+S",
                ))
                .child(c::with_tooltip(
                    c::overlay_action_button("Lock", OverlayTone::Neutral, || {}),
                    "Lock the screen  Super+Ctrl+L",
                ))
                .child(keycap_trigger("?", "Keyboard shortcuts  Super+K")),
        )
        .into_element()
}

/// A focusable host-element trigger: it takes `.tooltip(..)` directly, so
/// keyboard focus shows the hint too.
fn keycap_trigger(key: &'static str, hint: &'static str) -> impl IntoElement {
    view()
        .focusable(true)
        .role(AccessibilityRole::Button)
        .label(hint)
        .on_press(|| {})
        .w(px(tokens::CONTROL_MD))
        .h(px(tokens::CONTROL_MD))
        .items_center()
        .justify_center()
        .border(px(tokens::BORDER))
        .border_color(move || tokens::control_border())
        .transition_colors()
        .duration(tokens::DURATION_FAST)
        .hover(|s| s.bg(move || tokens::hover_fill()))
        .focus_visible(|s| s.border_color(move || tokens::focus_border()))
        .child(text(key).font_family(tokens::font_mono()))
        .tooltip(c::tooltip(hint))
}

pub fn hover_card() -> AnyElement {
    page()
        .child(lead(
            "A preview while the pointer rests on its trigger. Supplementary: what a keyboard \
             user needs stays reachable outside it.",
        ))
        .child(
            row().child(text("Theme by")).child(
                c::hover_card()
                    .trigger(text("@tokyo-night").text_color(move || tokens::accent()))
                    .child(c::popover_title("Tokyo Night"))
                    .child(text(
                        "A clean dark theme that celebrates the lights of Tokyo.",
                    ))
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
    // The gallery root provides the toaster and places its stack once; a
    // page mounted elsewhere (a test) makes and places its own.
    let (toaster, own) = match c::use_toaster() {
        Some(toaster) => (toaster, false),
        None => (c::toaster(), true),
    };
    let restored = signal(0_u32);
    let show = move |message: c::ToastMessage| {
        move || {
            toaster.show(message.clone());
        }
    };
    page()
        .child(lead(
            "Notices at the bottom right. They leave after 6 s; the pointer on the stack \
             pauses them. Errors stay until dismissed. Escape dismisses the newest.",
        ))
        .child(
            row()
                .child(c::overlay_action_button(
                    "Notify",
                    OverlayTone::Neutral,
                    show(c::toast("Screenshot saved").description("~/Pictures/screenshot.png")),
                ))
                .child(c::overlay_action_button(
                    "Success",
                    OverlayTone::Neutral,
                    show(c::toast("Update installed").kind(c::ToastKind::Success)),
                ))
                .child(c::overlay_action_button(
                    "Error",
                    OverlayTone::Danger,
                    show(
                        c::toast("Wi-Fi disconnected")
                            .description("Retrying in 30 s.")
                            .kind(c::ToastKind::Error),
                    ),
                ))
                .child(c::overlay_action_button(
                    "Close window",
                    OverlayTone::Neutral,
                    show(
                        c::toast("Window closed")
                            .timeout(Duration::from_secs(8))
                            .action("Undo", move || restored.update(|n| *n += 1)),
                    ),
                ))
                .child(c::overlay_action_button(
                    "Dismiss all",
                    OverlayTone::Neutral,
                    move || toaster.dismiss_all(),
                )),
        )
        .child(
            text(move || format!("Restored: {}", restored.get()))
                .text_size(px(tokens::TEXT_SM))
                .text_color(move || tokens::secondary()),
        )
        .child(own.then(|| toaster.view()))
        .into_element()
}
