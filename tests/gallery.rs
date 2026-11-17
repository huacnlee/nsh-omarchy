//! Gallery smoke tests: the shell renders and every page opens.

use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_testing::Headless;

pub fn start() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap()
}

#[test]
fn every_page_opens() -> Result<(), String> {
    let mut app = start();
    app.wait_for("Omarchy")?;
    for page in nsh_omarchy::gallery::registry::PAGES {
        app.press(&format!("label:{}", page.title))?;
        app.wait_for(&format!("text:{}", page.title))?;
    }
    Ok(())
}

#[test]
fn switches_theme() -> Result<(), String> {
    let mut app = start();
    app.wait_for("Omarchy")?;
    app.press("Light")?;
    app.wait_for("Flexoki Light")?;
    app.press("Dark")?;
    app.wait_for("Tokyo Night")?;
    Ok(())
}

/// The sidebar scrolls inside the window (a short window shows its last
/// pages), and the current page's row is revealed as the keys move it.
#[test]
fn the_sidebar_scrolls_to_the_current_page() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(Points(1180.0), Points(760.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap();
    app.wait_for("Omarchy")?;
    let last = nsh_omarchy::gallery::registry::PAGES.last().unwrap().title;
    app.press_key("end")?;
    app.wait_for(&format!("text:{last}"))?;
    let screen = app.screen();
    let nodes = screen.layout_nodes();
    // The sidebar's scroll view is bounded by the window, not its rows.
    let sidebar = nodes
        .iter()
        .find(|n| n.kind == nsh_core::ViewKind::Scroll)
        .ok_or("no scroll view")?;
    let bottom = sidebar.bounds.origin.y.0 + sidebar.bounds.size.height.0;
    assert!(
        bottom <= 760.5,
        "the sidebar runs past the window: {bottom}"
    );
    // The last page's row is selected and in view.
    let row = screen
        .press_target(&format!("label:{last}"))?
        .ok_or("no row for the last page")?;
    assert!(screen.selected(&format!("label:{last}"))?);
    let bounds = nodes
        .iter()
        .find(|n| n.id == row)
        .ok_or("row not laid out")?
        .bounds;
    let (top, end) = (bounds.origin.y.0, bounds.origin.y.0 + bounds.size.height.0);
    assert!(
        top >= sidebar.bounds.origin.y.0 && end <= bottom + 0.5,
        "the {last} row ({top}..{end}) is outside the sidebar (..{bottom})"
    );
    Ok(())
}
