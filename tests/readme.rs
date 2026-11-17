//! The Rust example of README.md, compiled and pressed through.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root};
use nsh_omarchy::{self as om, tokens};
use nsh_testing::Headless;

#[component]
pub fn App() -> impl IntoElement {
    // Draw with the Omarchy theme the system uses, and follow its changes.
    om::init();
    let saved = signal(0_u32);
    let notify = signal(true);
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
                .child(om::button_primary("Save").on_press(move || saved.update(|n| *n += 1))),
        )
        .child(om::body_text(move || {
            format!("Saved {} times", saved.get())
        }))
}

#[test]
fn readme_example_runs() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<App>())).unwrap();
    app.wait_for("Saved 0 times").unwrap();
    app.press("Save").unwrap();
    app.wait_for("Saved 1 times").unwrap();
}
