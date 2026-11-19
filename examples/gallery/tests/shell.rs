//! The gallery's shell: it starts on the first page and a press changes it.

use nsh_engine::{EngineConfig, Root};
use nsh_testing::Headless;

#[test]
fn a_press_changes_the_page() {
    let mut app = Headless::start(EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())).unwrap();
    app.wait_for("Button").unwrap();
    app.press("Checkbox").unwrap();
    app.wait_for("id:checkbox").unwrap();
}
