//! The choices pages, pressed through: presses and keys change the values.

use nsh_core::Checked;
use nsh_engine::{EngineConfig, Os, Points, Root, Size};
use nsh_testing::Headless;

/// The gallery on a headless macOS desktop (where Tab stops at controls),
/// opened at the page titled `page`.
fn open(page: &str) -> Result<Headless, String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1200.0), Points(900.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).map_err(|e| e.to_string())?;
    app.wait_for("role:button[Button]")?;
    app.press(&format!("role:button[{page}]"))?;
    Ok(app)
}

#[test]
fn checkbox_toggles() -> Result<(), String> {
    let mut app = open("Checkbox")?;
    app.wait_for("text:Show hidden files: off")?;
    app.press("label:Show hidden files")?;
    app.wait_for("text:Show hidden files: on")?;
    app.press("label:Show hidden files")?;
    app.wait_for("text:Show hidden files: off")?;
    assert!(!app.screen().enabled("label:Disabled")?);
    Ok(())
}

#[test]
fn select_all_is_indeterminate_over_a_partial_choice() -> Result<(), String> {
    let mut app = open("Checkbox")?;
    app.wait_for("text:1 of 3 chosen")?;
    app.press("label:Documents")?;
    app.wait_for("text:2 of 3 chosen")?;
    assert_eq!(
        app.screen().accessibility_state("label:All file types")?.checked,
        Some(Checked::Mixed)
    );
    app.press("label:All file types")?;
    app.wait_for("text:3 of 3 chosen")?;
    app.press("label:All file types")?;
    app.wait_for("text:0 of 3 chosen")?;
    Ok(())
}

#[test]
fn switch_toggles() -> Result<(), String> {
    let mut app = open("Switch")?;
    app.wait_for("text:Wi-Fi off, Bluetooth on")?;
    app.press("label:Wi-Fi")?;
    app.wait_for("text:Wi-Fi on, Bluetooth on")?;
    app.press("label:Bluetooth")?;
    app.wait_for("text:Wi-Fi on, Bluetooth off")?;
    assert!(!app.screen().enabled("label:Airplane mode")?);
    Ok(())
}

#[test]
fn radio_press_chooses() -> Result<(), String> {
    let mut app = open("Radio")?;
    app.wait_for("text:Appearance: dark")?;
    app.press("label:System")?;
    app.wait_for("text:Appearance: system")?;
    app.press("label:Roomy")?;
    app.wait_for("text:Density: roomy")?;
    assert!(!app.screen().enabled("label:High contrast")?);
    Ok(())
}

#[test]
fn radio_arrow_keys_choose() -> Result<(), String> {
    let mut app = open("Radio")?;
    app.wait_for("text:Appearance: dark")?;
    // The page's last stops: the density group, then the appearance group,
    // each entered at its checked radio.
    app.press_key("shift+tab")?;
    app.press_key("shift+tab")?;
    app.press_key("down")?;
    app.wait_for("text:Appearance: system")?;
    // Past the disabled option, around to the first.
    app.press_key("down")?;
    app.wait_for("text:Appearance: light")?;
    assert!(app.screen().shows("Density: compact"));
    Ok(())
}

#[test]
fn progress_follows_its_value() -> Result<(), String> {
    let mut app = open("Progress")?;
    app.wait_for("text:35%")?;
    // The page's last stop: the slider driving the bar.
    app.press_key("shift+tab")?;
    app.press_key("end")?;
    app.wait_for("text:100%")?;
    app.press_key("left")?;
    app.wait_for("text:99%")?;
    Ok(())
}

#[test]
fn slider_page_steps() -> Result<(), String> {
    let mut app = open("Slider")?;
    app.wait_for("text:Volume 40")?;
    app.wait_for("text:20 – 80")?;
    // The page's last stop: the range's end thumb (the disabled slider is
    // none).
    app.press_key("shift+tab")?;
    app.press_key("left")?;
    app.wait_for("text:20 – 79")?;
    Ok(())
}
