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
