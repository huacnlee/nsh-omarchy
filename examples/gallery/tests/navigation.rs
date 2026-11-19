//! The navigation pages, pressed through.

use nsh_engine::{EngineConfig, Environment, Os, Points, Root, Size};
use nsh_testing::Headless;

/// The gallery on a desktop-sized macOS screen, opened on `page`.
fn gallery(page: &str) -> Result<Headless, String> {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(900.0)),
        environment: Environment {
            os: Os::MacOs,
            ..Environment::default()
        },
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    })
    .map_err(|e| e.to_string())?;
    app.wait_for("Button")?;
    app.press(&format!("role:button[{page}]"))?;
    Ok(app)
}

#[test]
fn tabs_select_by_press() -> Result<(), String> {
    let mut app = gallery("Tabs")?;
    app.wait_for("text:tab: overview")?;
    app.press("role:tab[Activity]")?;
    app.wait_for("text:tab: activity")?;
    app.wait_for("12 deploys this week.")?;
    assert!(!app.screen().enabled("role:tab[Archived]")?);
    app.wait_for("text:range: day")?;
    app.press("role:tab[Month]")?;
    app.wait_for("text:range: month")?;
    Ok(())
}

#[test]
fn the_menu_chooses_and_toggles() -> Result<(), String> {
    let mut app = gallery("Menu")?;
    app.wait_for("text:chose: nothing · sidebar: on · wrap: off")?;
    app.press("role:button[File]")?;
    app.wait_for("Document")?;
    app.press("Word wrap")?;
    app.wait_for("text:chose: nothing · sidebar: on · wrap: on")?;
    app.press("text:Open…")?;
    app.wait_for("text:chose: Open · sidebar: on · wrap: on")?;
    app.press("role:button[File]")?;
    app.wait_for("Document")?;
    app.press("text:Delete")?;
    app.wait_for("text:chose: Delete · sidebar: on · wrap: on")?;
    Ok(())
}

#[test]
fn the_context_menu_opens_on_a_right_click() -> Result<(), String> {
    let mut app = gallery("Context menu")?;
    app.wait_for("text:chose: nothing")?;
    app.right_click("label:Context area")?;
    app.wait_for("Rename")?;
    app.press("text:Rename")?;
    app.wait_for("text:chose: Rename")?;
    Ok(())
}

#[test]
fn accordions_open_items() -> Result<(), String> {
    let mut app = gallery("Accordion")?;
    app.wait_for("text:open: shipping")?;
    app.press("role:button[Can I return an item?]")?;
    app.wait_for("text:open: returns")?;
    app.press("role:button[Display]")?;
    app.press("role:button[Sound]")?;
    app.wait_for("text:several open: display, sound")?;
    Ok(())
}

#[test]
fn collapsibles_open() -> Result<(), String> {
    let mut app = gallery("Collapsible")?;
    app.wait_for("text:collapsible: closed")?;
    app.press("label:Advanced options")?;
    app.wait_for("text:collapsible: open")?;
    app.wait_for("text:Compression: zstd, level 3")?;
    app.press("label:Release notes")?;
    app.wait_for("text:0.1: the first release.")?;
    assert!(!app.screen().enabled("label:Locked")?);
    Ok(())
}

#[test]
fn pagination_moves_between_pages() -> Result<(), String> {
    let mut app = gallery("Pagination")?;
    app.wait_for("page 1 of 5")?;
    app.press("label:Next page >> nth=0")?;
    app.wait_for("page 2 of 5")?;
    app.press("label:Next page >> nth=1")?;
    app.wait_for("results page 8 of 40")?;
    Ok(())
}

#[test]
fn the_sidebar_selects_items() -> Result<(), String> {
    let mut app = gallery("Sidebar")?;
    app.wait_for("text:mailbox: inbox")?;
    app.press("role:tab[Drafts]")?;
    app.wait_for("text:mailbox: drafts")?;
    assert!(!app.screen().enabled("role:tab[Archive]")?);
    Ok(())
}
