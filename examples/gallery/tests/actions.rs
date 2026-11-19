//! The actions pages, pressed through.

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
fn the_button_page_counts_activations() -> Result<(), String> {
    let mut app = open("Button")?;
    app.wait_for("text:Activations: 0")?;
    app.press("text:Apply >> nth=0")?;
    app.press("text:Apply >> nth=1")?;
    app.press("label:Add panel")?;
    app.wait_for("text:Activations: 3")?;
    assert!(!app.screen().enabled("label:Add panel unavailable")?);
    app.press("role:button[Pin]")?;
    app.wait_for("text:Pinned: on")?;
    assert!(app.screen().selected("role:button[Pinned]")?);
    app.press("role:button[Reset]")?;
    app.wait_for("text:Activations: 0")?;
    Ok(())
}

#[test]
fn the_toggle_page_follows_its_toggles() -> Result<(), String> {
    let mut app = open("Toggle")?;
    app.wait_for("text:Favorite panel: off")?;
    app.press("role:button[Favorite panel]")?;
    app.wait_for("text:Favorite panel: on")?;
    assert!(!app.screen().enabled("role:button[Locked on]")?);
    Ok(())
}

#[test]
fn the_toggle_group_page_filters_and_formats() -> Result<(), String> {
    let mut app = open("Toggle group")?;
    app.wait_for("text:4 articles")?;
    app.press("role:button[Draft]")?;
    app.wait_for("text:2 articles")?;
    assert!(!app.shows("Autumn release notes"));
    app.press("role:button[In review]")?;
    app.press("role:button[Published]")?;
    app.wait_for("No articles match.")?;
    app.press("role:button[Italic]")?;
    app.wait_for("text:Bold off · Italic on · Underline off")?;
    app.press("role:button[Align right]")?;
    app.wait_for("text:Aligned: right")?;
    Ok(())
}

#[test]
fn the_button_group_page_presses_its_buttons() -> Result<(), String> {
    let mut app = open("Button group")?;
    app.wait_for("text:Toolbar is placed at the top.")?;
    app.press("role:button[Bottom]")?;
    app.wait_for("text:Toolbar is placed at the bottom.")?;
    assert!(app.screen().selected("role:button[Bottom]")?);
    app.press("label:Zoom in")?;
    app.wait_for("text:Zoom: 110%")?;
    app.press("role:button[Reset zoom]")?;
    app.wait_for("text:Zoom: 100%")?;
    assert!(!app.screen().enabled("role:button[Paste]")?);
    Ok(())
}

#[test]
fn the_link_page_reports_an_app_link() -> Result<(), String> {
    let mut app = open("Link")?;
    app.wait_for("text:Opened: nothing yet")?;
    app.press("role:link[Release notes]")?;
    app.wait_for("text:Opened: /releases")?;
    Ok(())
}
