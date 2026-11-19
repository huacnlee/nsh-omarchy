//! The Rust example of README.md, compiled and pressed through.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_omarchy::{self as om, tokens};
use nsh_testing::Headless;

#[component]
pub fn App() -> impl IntoElement {
    // Draw with the system's Omarchy theme, and follow it as it changes.
    om::init();
    let notify = signal(true);
    let saved = signal(0_u32);
    view()
        .flex_1()
        .p(px(tokens::SPACE_6))
        .gap(px(tokens::SPACE_4))
        .bg(move || tokens::background())
        .child(
            om::panel("Settings")
                .child(
                    om::switch("Notifications")
                        .checked(move || notify.get())
                        .on_change(move |on: &bool| notify.set(*on)),
                )
                .child(om::button("Save").primary().on_press(move || {
                    saved.update(|n| *n += 1);
                    om::toast(om::Toast::new("Settings saved"));
                })),
        )
        .child(om::body(move || format!("Saved {} times", saved.get())))
        .child(om::toaster())
}

#[test]
fn readme_example_runs() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<App>())).unwrap();
    app.wait_for("text:Saved 0 times").unwrap();
    app.press("role:switch[Notifications]").unwrap();
    app.press("role:button[Save]").unwrap();
    app.wait_for("text:Saved 1 times").unwrap();
    app.wait_for("text:Settings saved").unwrap();
}
