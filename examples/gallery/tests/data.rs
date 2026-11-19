//! The data pages, pressed through in the gallery.

use std::time::Duration;

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Root, Size};
use nsh_testing::Headless;

const WAIT: Duration = Duration::from_secs(5);

/// The gallery, on the page titled `page`.
fn gallery(page: &str) -> Result<Headless, String> {
    let config = EngineConfig {
        viewport: Size::new(px(1400.0), px(1000.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    };
    let mut app = Headless::start(config).map_err(|e| e.to_string())?;
    app.first_render()?;
    app.press(page)?;
    Ok(app)
}

#[test]
fn the_table_page_selects_and_sorts() -> Result<(), String> {
    let mut app = gallery("Table")?;
    app.wait_for("text:selected pid 1207")?;
    app.press("label:nvim 1221")?;
    app.wait_for("text:selected pid 1221")?;
    app.wait_for("text:sort none, selected pid none")?;
    app.press("id:sort:cpu")?;
    app.wait_for("text:sort cpu Ascending, selected pid none")?;
    Ok(())
}

#[test]
fn the_list_page_selects_by_id() -> Result<(), String> {
    let mut app = gallery("List")?;
    app.wait_for("label:Item 1")?;
    app.press("label:Item 4")?;
    app.wait_for("text:selected item 4")?;
    app.scroll("id:items", px(2800.0))?;
    app.wait_for("label:Item 105")?;
    app.wait_for_gone("label:Item 4", WAIT)?;
    Ok(())
}

#[test]
fn the_tree_page_folds() -> Result<(), String> {
    let mut app = gallery("Tree")?;
    app.wait_for("text:selected hypr, expanded config")?;
    app.press("role:treeitem[themes]")?;
    app.wait_for("text:selected themes, expanded config themes")?;
    app.wait_for("role:treeitem[catppuccin]")?;
    Ok(())
}

#[test]
fn the_resizable_page_reports_sizes() -> Result<(), String> {
    let mut app = gallery("Resizable")?;
    app.wait_for("text:sizes as laid out")?;
    app.wait_for("text:Terminal")?;
    app.drag("label:Resize panels horizontally >> nth=0", Point::new(px(40.0), px(0.0)))?;
    app.wait_for("text:/sizes 240 /")?;
    Ok(())
}
