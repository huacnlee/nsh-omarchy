//! The overlays pages, pressed through: dialogs, sheets, popovers,
//! tooltips, hover cards and toasts open and dismiss.

use std::time::{Duration, SystemTime};

use nsh_engine::{Clock, EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_testing::{DialogAnswer, Headless};

const GONE: Duration = Duration::from_secs(2);

fn config(manual: bool) -> EngineConfig {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        environment: Environment {
            os: Os::Linux,
            idiom: Idiom::Desktop,
            ..Environment::default()
        },
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    };
    if manual {
        config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    }
    config
}

/// The gallery on page `title` (its sidebar button).
fn open_page(title: &str, manual: bool) -> Result<Headless, String> {
    let mut app = Headless::start(config(manual)).unwrap();
    app.wait_for("label:Button")?;
    app.press(&format!("label:{title}"))?;
    app.wait_for(&format!("text:{title}"))?;
    Ok(app)
}

#[test]
fn dialog_opens_and_closes_by_cancel_escape_and_action() -> Result<(), String> {
    let mut app = open_page("Dialog", false)?;
    let body = "text:The name shows in the bar and the workspace switcher.";

    app.press("label:Rename workspace")?;
    app.wait_for(body)?;
    app.press("label:Cancel")?;
    app.wait_for_gone(body, GONE)?;
    app.wait_for("text:Last: dismissed")?;

    app.press("label:Rename workspace")?;
    app.wait_for(body)?;
    app.press_key("escape")?;
    app.wait_for_gone(body, GONE)?;

    app.press("label:Rename workspace")?;
    app.wait_for(body)?;
    app.press("label:Rename")?;
    app.wait_for_gone(body, GONE)?;
    app.wait_for("text:Last: renamed")?;
    Ok(())
}

#[test]
fn alert_dialog_confirms_and_cancels() -> Result<(), String> {
    let mut app = open_page("Alert dialog", false)?;
    let body = "text:Its 2 windows move to workspace 1. This cannot be undone.";

    app.press("label:Delete workspace")?;
    app.wait_for(body)?;
    assert!(app.screen().matches("role:alertdialog")?);
    app.press_key("escape")?;
    app.wait_for_gone(body, GONE)?;
    app.wait_for("text:Last: kept")?;

    app.press("label:Delete workspace")?;
    app.wait_for(body)?;
    app.press("label:Delete")?;
    app.wait_for_gone(body, GONE)?;
    app.wait_for("text:Last: deleted")?;
    Ok(())
}

#[test]
fn native_confirm_reports_the_choice() -> Result<(), String> {
    let mut app = open_page("Alert dialog", false)?;
    app.on_dialog(|_| DialogAnswer::Press("Delete".to_owned()));
    app.press("label:Native confirm")?;
    app.wait_for("text:Last: native: deleted")?;
    Ok(())
}

#[test]
fn sheet_opens_and_closes() -> Result<(), String> {
    let mut app = open_page("Sheet", false)?;

    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press("label:Close")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;
    app.wait_for("text:Last: sheet dismissed")?;

    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press("label:Apply")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;
    app.wait_for("text:Last: applied")?;

    app.press("label:Bottom sheet")?;
    app.wait_for("text:3 items")?;
    app.press_key("escape")?;
    app.wait_for_gone("text:3 items", GONE)?;
    app.wait_for("text:Last: bottom dismissed")?;
    Ok(())
}

#[test]
fn popover_toggles_from_its_trigger_and_closes_on_escape() -> Result<(), String> {
    let mut app = open_page("Popover", false)?;
    let content = "text:2 windows · code, terminal";

    app.press("label:Workspace details")?;
    app.wait_for(content)?;
    app.wait_for("text:Workspace 3")?;
    app.press_key("escape")?;
    app.wait_for_gone(content, GONE)?;

    app.press("label:Workspace details")?;
    app.wait_for(content)?;
    app.press("label:Workspace details")?;
    app.wait_for_gone(content, GONE)?;
    Ok(())
}

#[test]
fn tooltip_shows_after_hover_delay() -> Result<(), String> {
    let mut app = open_page("Tooltip", true)?;
    app.hover("label:Lock")?;
    assert!(!app.screen().shows("Lock the screen"), "not before the delay");
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:Lock the screen")?;
    app.wait_for("text:super-ctrl-l")?;
    Ok(())
}

#[test]
fn hover_card_shows_on_hover() -> Result<(), String> {
    let mut app = open_page("Hover card", true)?;
    app.hover("label:@tokyo-night")?;
    assert!(!app.screen().shows("Built in · 16 colors"), "not before the delay");
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:Built in · 16 colors")?;
    Ok(())
}

#[test]
fn toast_expires_after_its_timeout() -> Result<(), String> {
    let mut app = open_page("Toast", true)?;
    app.press("label:Notify")?;
    app.wait_for("text:Screenshot saved")?;
    app.run_for(Duration::from_secs(3))?;
    assert!(app.screen().shows("Screenshot saved"), "still shown at 3 s");
    app.run_for(Duration::from_secs(4))?;
    app.wait_for_gone("text:Screenshot saved", GONE)?;
    Ok(())
}

#[test]
fn error_toast_stays_until_dismissed() -> Result<(), String> {
    let mut app = open_page("Toast", true)?;
    app.press("label:Error")?;
    app.wait_for("text:Wi-Fi disconnected")?;
    app.run_for(Duration::from_secs(20))?;
    assert!(app.screen().shows("Wi-Fi disconnected"), "an error persists");
    app.press("label:Dismiss notification")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Wi-Fi disconnected", GONE)?;
    Ok(())
}

#[test]
fn toast_action_runs_and_dismisses() -> Result<(), String> {
    let mut app = open_page("Toast", true)?;
    app.press("label:Close window")?;
    app.wait_for("text:Window closed")?;
    app.press("label:Undo")?;
    app.wait_for("text:Restored: 1")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Window closed", GONE)?;
    Ok(())
}

#[test]
fn dismiss_toast_removes_the_last_one() -> Result<(), String> {
    let mut app = open_page("Toast", true)?;
    app.press("label:Close window")?;
    app.wait_for("text:Window closed")?;
    app.press("label:Dismiss last")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Window closed", GONE)?;
    Ok(())
}

#[test]
fn hovering_the_stack_pauses_toasts() -> Result<(), String> {
    let mut app = open_page("Toast", true)?;
    app.press("label:Success")?;
    app.wait_for("text:Update installed")?;
    app.hover("text:Update installed")?;
    app.run_for(Duration::from_secs(10))?;
    assert!(app.screen().shows("Update installed"), "paused while hovered");
    Ok(())
}
