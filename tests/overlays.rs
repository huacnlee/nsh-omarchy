//! Overlay components, driven through their gallery pages: dialogs, sheets,
//! popovers, tooltips, hover cards and toasts open and dismiss.

use std::time::{Duration, SystemTime};

use nsh_engine::{Clock, EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_testing::{DialogAnswer, Headless, Screen};

const GONE: Duration = Duration::from_secs(2);

/// A desktop (Linux, the Omarchy target): a mouse to hover with.
fn desktop() -> Environment {
    Environment {
        os: Os::Linux,
        idiom: Idiom::Desktop,
        ..Environment::default()
    }
}

fn start() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        environment: desktop(),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap()
}

/// With a manual clock: time moves only with `run_for`.
fn start_manual() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        environment: desktop(),
        clock: Clock::manual(SystemTime::UNIX_EPOCH),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap()
}

fn open_page(app: &mut Headless, title: &str) -> Result<(), String> {
    app.wait_for("Omarchy")?;
    app.press(&format!("label:{title}"))?;
    app.wait_for(&format!("text:{title}"))
}

#[test]
fn dialog_opens_and_closes_by_cancel_escape_and_action() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Dialog")?;
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
    let mut app = start();
    open_page(&mut app, "Alert dialog")?;
    let body = "text:Its 2 windows move to workspace 1. This cannot be undone.";

    app.press("label:Delete workspace")?;
    app.wait_for(body)?;
    assert!(app.screen().matches("role:alertdialog")? || app.screen().shows("Delete workspace 3?"));
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
    let mut app = start();
    open_page(&mut app, "Alert dialog")?;
    app.on_dialog(|_| DialogAnswer::Press("Delete".to_owned()));
    app.press("label:Native confirm")?;
    app.wait_for("text:Last: native: deleted")?;
    Ok(())
}

#[test]
fn sheet_opens_and_closes() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Sheet")?;

    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press("label:Close")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;
    app.wait_for("text:Last: sheet dismissed")?;

    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press_key("escape")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;

    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press("label:Apply")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;
    app.wait_for("text:Last: applied")?;

    app.press("label:Bottom sheet")?;
    app.wait_for("text:3 items")?;
    app.press("label:Close")?;
    app.wait_for_gone("text:3 items", GONE)?;
    Ok(())
}

#[test]
fn popover_toggles_from_its_trigger_and_closes_on_escape() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Popover")?;
    let content = "text:2 windows · code, terminal";

    app.press("label:Workspace details")?;
    app.wait_for(content)?;
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
    let mut app = start_manual();
    open_page(&mut app, "Tooltip")?;
    app.hover("label:Save")?;
    assert!(!app.screen().shows("Save  Ctrl+S"), "not before the delay");
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:Save  Ctrl+S")?;
    Ok(())
}

#[test]
fn hover_card_shows_on_hover() -> Result<(), String> {
    let mut app = start_manual();
    open_page(&mut app, "Hover card")?;
    app.hover("text:@tokyo-night")?;
    assert!(
        !app.screen().shows("Built in · 16 colors"),
        "not before the delay"
    );
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:Built in · 16 colors")?;
    Ok(())
}

#[test]
fn toast_expires_after_its_timeout() -> Result<(), String> {
    let mut app = start_manual();
    open_page(&mut app, "Toast")?;
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
    let mut app = start_manual();
    open_page(&mut app, "Toast")?;
    app.press("label:Error")?;
    app.wait_for("text:Wi-Fi disconnected")?;
    app.run_for(Duration::from_secs(20))?;
    assert!(
        app.screen().shows("Wi-Fi disconnected"),
        "an error persists"
    );
    app.press("label:Dismiss notification")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Wi-Fi disconnected", GONE)?;
    Ok(())
}

#[test]
fn toast_action_runs_and_dismisses() -> Result<(), String> {
    let mut app = start_manual();
    open_page(&mut app, "Toast")?;
    app.press("label:Close window")?;
    app.wait_for("text:Window closed")?;
    app.press("label:Undo")?;
    app.wait_for("text:Restored: 1")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Window closed", GONE)?;
    Ok(())
}

#[test]
fn escape_dismisses_the_newest_toast() -> Result<(), String> {
    let mut app = start_manual();
    open_page(&mut app, "Toast")?;
    app.press("label:Error")?;
    app.wait_for("text:Wi-Fi disconnected")?;
    app.press_key("escape")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Wi-Fi disconnected", GONE)?;
    Ok(())
}

#[test]
fn hovering_the_stack_pauses_toasts() -> Result<(), String> {
    let mut app = start_manual();
    open_page(&mut app, "Toast")?;
    app.press("label:Success")?;
    app.wait_for("text:Update installed")?;
    app.hover("text:Update installed")?;
    app.run_for(Duration::from_secs(10))?;
    assert!(
        app.screen().shows("Update installed"),
        "paused while hovered"
    );
    Ok(())
}

/// How many times the keyboard focus was sent to the view `selector` names.
fn focus_count(app: &Headless, selector: &str) -> Result<usize, String> {
    focus_count_on(app.screen(), selector)
}

fn focus_count_on(screen: &Screen, selector: &str) -> Result<usize, String> {
    let node = screen
        .press_target(selector)?
        .ok_or_else(|| format!("no view for {selector}"))?;
    Ok(screen
        .commands()
        .iter()
        .filter(|(n, c)| *n == node && *c == nsh_core::Command::Focus)
        .count())
}

/// Waits until the focus was sent to `selector` at least `count` times.
fn wait_for_focus(app: &mut Headless, selector: &str, count: usize) -> Result<(), String> {
    let what = format!("focus on {selector}");
    app.wait_until(&what, GONE, |screen| {
        focus_count_on(screen, selector).is_ok_and(|n| n >= count)
    })
}

#[test]
fn dialog_focuses_its_default_action_and_returns_focus() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Dialog")?;
    let body = "text:The name shows in the bar and the workspace switcher.";
    let before = focus_count(&app, "label:Rename workspace")?;
    app.press("label:Rename workspace")?;
    app.wait_for(body)?;
    assert!(
        focus_count(&app, "label:Rename")? >= 1,
        "the primary action takes the first focus"
    );
    app.press_key("escape")?;
    app.wait_for_gone(body, GONE)?;
    assert_eq!(
        focus_count(&app, "label:Rename workspace")?,
        before + 1,
        "the focus returns to the trigger"
    );
    Ok(())
}

#[test]
fn alert_dialog_focuses_cancel_first() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Alert dialog")?;
    app.press("label:Delete workspace")?;
    app.wait_for("text:Delete workspace 3?")?;
    assert!(
        focus_count(&app, "label:Cancel")? >= 1,
        "Cancel takes the first focus"
    );
    assert_eq!(focus_count(&app, "label:Delete")?, 0);
    Ok(())
}

#[test]
fn sheet_returns_focus_to_its_trigger() -> Result<(), String> {
    let mut app = start();
    open_page(&mut app, "Sheet")?;
    let before = focus_count(&app, "label:Display settings")?;
    app.press("label:Display settings")?;
    app.wait_for("text:Refresh rate")?;
    wait_for_focus(&mut app, "label:Close", 1)?;
    app.press_key("escape")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;
    wait_for_focus(&mut app, "label:Display settings", before + 1)?;
    Ok(())
}
