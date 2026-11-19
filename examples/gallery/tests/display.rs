//! The display pages: each shows its demos, and the interactive ones respond.

use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_omarchy::IconName;
use nsh_testing::Headless;

fn gallery() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(1600.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    })
    .unwrap()
}

/// Opens the page titled `title` from the sidebar.
fn open(app: &mut Headless, title: &str) -> Result<(), String> {
    app.wait_for("label:Button")?;
    app.press(&format!("label:{title}"))?;
    app.wait_for(&format!("text:{title}"))
}

fn expect_texts(app: &mut Headless, texts: &[&str]) -> Result<(), String> {
    for t in texts {
        app.wait_for(&format!("text:{t}"))?;
    }
    Ok(())
}

#[test]
fn panel_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Panel")?;
    expect_texts(
        &mut app,
        &["Workspace", "SYNCED", "Release checklist", "Updates", "omarchy-theme-set tokyo-night"],
    )
}

#[test]
fn separator_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Separator")?;
    expect_texts(&mut app, &["Above the rule", "Below the rule", "Files", "View"])
}

#[test]
fn badge_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Badge")?;
    expect_texts(
        &mut app,
        &["DEFAULT", "PRIMARY", "SUCCESS", "WARNING", "DANGER", "ONLINE", "LARGE", "PASSING"],
    )
}

#[test]
fn keycap_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Keycap")?;
    expect_texts(&mut app, &["esc", "tab", "super", "space", "Open the launcher", "shift"])?;
    assert!(app.screen().count("role:keyboardkey")? >= 10);
    Ok(())
}

#[test]
fn empty_state_action() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Empty state")?;
    expect_texts(&mut app, &["No projects yet", "Inbox zero"])?;
    app.press("label:Create project")?;
    app.wait_for("text:Project created")?;
    assert!(!app.shows("No projects yet"));
    Ok(())
}

#[test]
fn avatar_page_shows_initials() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Avatar")?;
    app.wait_for("label:Grace Hopper")?;
    expect_texts(&mut app, &["AL", "GH", "LT", "HU"])
}

#[test]
fn icon_page_lists_the_set() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Icon")?;
    for name in IconName::ALL {
        app.wait_for(&format!("text:{}", name.name()))?;
    }
    app.wait_for("label:Encrypted")
}

#[test]
fn typography_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Typography")?;
    expect_texts(
        &mut app,
        &["Heading", "Title", "Caption: secondary hints and timestamps", "omarchy-update"],
    )
}

#[test]
fn markdown_renders_and_reports_links() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Markdown")?;
    expect_texts(&mut app, &["Release notes", "Highlights", "Press a link"])?;
    app.press("text:links")?;
    app.wait_for("text:Link pressed: https://omarchy.org")
}
