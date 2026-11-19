//! Overlays and feedback through the public API: dialogs, sheets, popovers,
//! hover cards, tooltips, toasts and the platform's confirm.

use std::time::{Duration, SystemTime};

use nsh_core::prelude::*;
use nsh_engine::{Clock, EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_omarchy::{self as om, tokens};
use nsh_testing::{DialogAnswer, Headless};

const GONE: Duration = Duration::from_secs(2);

/// A desktop (Linux, the Omarchy target): a mouse to hover with.
fn config(root: Root, manual: bool) -> EngineConfig {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1000.0), Points(700.0)),
        environment: Environment {
            os: Os::Linux,
            idiom: Idiom::Desktop,
            ..Environment::default()
        },
        ..EngineConfig::new(root)
    };
    if manual {
        config.clock = Clock::manual(SystemTime::UNIX_EPOCH);
    }
    config
}

fn start(root: Root) -> Headless {
    Headless::start(config(root, false)).unwrap()
}

/// With a manual clock: time moves only with `run_for`.
fn start_manual(root: Root) -> Headless {
    Headless::start(config(root, true)).unwrap()
}

fn page() -> View {
    view().flex_1().p(px(tokens::SPACE_6)).gap(px(tokens::SPACE_4))
}

fn status(last: Signal<SharedString>) -> impl IntoElement {
    text(move || format!("Last: {}", last.get()))
}

// ---- Dialog ---------------------------------------------------------------

#[component]
fn DialogApp() -> impl IntoElement {
    let open = signal(false);
    let last = signal(SharedString::from("none"));
    page()
        .child(om::button("Rename workspace").on_press(move || open.set(true)))
        .child(status(last))
        .child(
            om::dialog("Rename workspace")
                .description("Shown in the bar and the switcher.")
                .open(move || open.get())
                .on_open_change(move |o: &bool| {
                    open.set(*o);
                    last.set(format!("open {o}").into());
                })
                .footer(
                    view()
                        .flex_row()
                        .child(om::button("Cancel").on_press(move || open.set(false)))
                        .child(om::button("Save").primary().on_press(move || {
                            last.set("saved".into());
                            open.set(false);
                        })),
                )
                .child(text("3: code")),
        )
}

#[test]
fn a_dialog_opens_and_closes_by_escape_and_its_footer() -> Result<(), String> {
    let mut app = start(Root::of::<DialogApp>());
    app.wait_for("label:Rename workspace")?;
    app.press("label:Rename workspace")?;
    app.wait_for("text:3: code")?;
    app.wait_for("text:Shown in the bar and the switcher.")?;
    app.press_key("escape")?;
    app.wait_for_gone("text:3: code", GONE)?;
    app.wait_for("text:Last: open false")?;

    app.press("label:Rename workspace")?;
    app.wait_for("text:3: code")?;
    app.press("label:Save")?;
    app.wait_for_gone("text:3: code", GONE)?;
    app.wait_for("text:Last: saved")?;

    app.press("label:Rename workspace")?;
    app.wait_for("text:3: code")?;
    app.press("label:Cancel")?;
    app.wait_for_gone("text:3: code", GONE)?;
    Ok(())
}

#[component]
fn StickyDialogApp() -> impl IntoElement {
    let open = signal(true);
    page().child(text(move || format!("Open: {}", open.get()))).child(
        om::dialog("Sticky")
            .open(move || open.get())
            .on_open_change(move |o: &bool| open.set(*o))
            .close_on_escape(false)
            .close_on_backdrop_press(false)
            .child(text("Stays")),
    )
}

#[test]
fn a_dialog_can_ignore_escape() -> Result<(), String> {
    let mut app = start_manual(Root::of::<StickyDialogApp>());
    app.wait_for("text:Stays")?;
    app.press_key("escape")?;
    app.run_for(Duration::from_millis(300))?;
    assert!(app.screen().shows("Open: true"), "no request to close");
    assert!(app.screen().shows("Stays"), "close_on_escape(false) keeps it");
    Ok(())
}

// ---- AlertDialog ----------------------------------------------------------

#[component]
fn AlertApp() -> impl IntoElement {
    let open = signal(false);
    let last = signal(SharedString::from("none"));
    page()
        .child(om::button("Delete workspace").danger().on_press(move || open.set(true)))
        .child(status(last))
        .child(
            om::alert_dialog("Delete workspace?", "Delete")
                .description("Its windows move to workspace 1.")
                .danger()
                .open(move || open.get())
                .on_open_change(move |o: &bool| open.set(*o))
                .on_action(move || last.set("deleted".into())),
        )
}

#[test]
fn an_alert_dialog_cancels_and_confirms() -> Result<(), String> {
    let mut app = start(Root::of::<AlertApp>());
    app.wait_for("label:Delete workspace")?;
    app.press("label:Delete workspace")?;
    app.wait_for("text:Delete workspace?")?;
    assert!(app.screen().matches("role:alertdialog")?);
    app.press("label:Cancel")?;
    app.wait_for_gone("text:Delete workspace?", GONE)?;
    assert!(app.screen().shows("Last: none"));

    app.press("label:Delete workspace")?;
    app.wait_for("text:Its windows move to workspace 1.")?;
    app.press("label:Delete")?;
    app.wait_for_gone("text:Delete workspace?", GONE)?;
    app.wait_for("text:Last: deleted")?;
    Ok(())
}

// ---- Sheet ----------------------------------------------------------------

#[component]
fn SheetApp() -> impl IntoElement {
    let open = signal(false);
    let bottom = signal(false);
    page()
        .child(om::button("Open settings").on_press(move || open.set(true)))
        .child(om::button("Clipboard").on_press(move || bottom.set(true)))
        .child(
            om::sheet("Quick settings")
                .description("Applies at once.")
                .open(move || open.get())
                .on_open_change(move |o: &bool| open.set(*o))
                .footer(om::button("Done").primary().on_press(move || open.set(false)))
                .child(text("Refresh rate")),
        )
        .child(
            om::sheet("History")
                .placement(Placement::Bottom)
                .close_label("Close history")
                .open(move || bottom.get())
                .on_open_change(move |o: &bool| bottom.set(*o))
                .child(text("3 items")),
        )
}

#[test]
fn a_sheet_closes_by_its_button_escape_and_footer() -> Result<(), String> {
    let mut app = start(Root::of::<SheetApp>());
    app.wait_for("label:Open settings")?;
    app.press("label:Open settings")?;
    app.wait_for("text:Refresh rate")?;
    app.wait_for("text:Applies at once.")?;
    app.press("label:Close")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;

    app.press("label:Open settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press_key("escape")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;

    app.press("label:Open settings")?;
    app.wait_for("text:Refresh rate")?;
    app.press("label:Done")?;
    app.wait_for_gone("text:Refresh rate", GONE)?;

    app.press("label:Clipboard")?;
    app.wait_for("text:3 items")?;
    app.press("label:Close history")?;
    app.wait_for_gone("text:3 items", GONE)?;
    Ok(())
}

// ---- Popover, HoverCard, Tooltip ------------------------------------------

#[component]
fn PopupApp() -> impl IntoElement {
    let changes = signal(0_u32);
    page()
        .child(
            view()
                .flex_row()
                .gap(px(tokens::SPACE_2))
                .child(
                    om::popover()
                        .trigger(om::button("Details"))
                        .title("Battery")
                        .on_open_change(move |_: &bool| changes.update(|n| *n += 1))
                        .child(text("82%, about 5 hours left.")),
                )
                .child(
                    om::button("Save")
                        .ghost()
                        .tooltip(om::tooltip("Save").shortcut("mod-s")),
                )
                .child(
                    om::hover_card()
                        .trigger(om::button("@dhh").ghost())
                        .child(text("Creator of Omarchy.")),
                ),
        )
        .child(text(move || format!("Changes: {}", changes.get())))
}

#[test]
fn a_popover_toggles_from_its_trigger_and_escape_closes_it() -> Result<(), String> {
    let mut app = start(Root::of::<PopupApp>());
    let content = "text:82%, about 5 hours left.";
    app.wait_for("label:Details")?;
    app.press("label:Details")?;
    app.wait_for(content)?;
    app.wait_for("text:Battery")?;
    app.wait_for("text:Changes: 1")?;
    app.press_key("escape")?;
    app.wait_for_gone(content, GONE)?;
    app.wait_for("text:Changes: 2")?;

    app.press("label:Details")?;
    app.wait_for(content)?;
    app.press("label:Details")?;
    app.wait_for_gone(content, GONE)?;
    Ok(())
}

#[test]
fn a_tooltip_shows_after_the_pointer_rests() -> Result<(), String> {
    let mut app = start_manual(Root::of::<PopupApp>());
    app.wait_for("label:Save")?;
    app.hover("label:Save")?;
    assert!(!app.screen().shows("ctrl-s"), "not before the delay");
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:ctrl-s")?;
    Ok(())
}

#[test]
fn a_hover_card_shows_after_the_pointer_rests() -> Result<(), String> {
    let mut app = start_manual(Root::of::<PopupApp>());
    app.wait_for("label:@dhh")?;
    app.hover("label:@dhh")?;
    assert!(!app.screen().shows("Creator of Omarchy."), "not before the delay");
    app.run_for(Duration::from_millis(600))?;
    app.wait_for("text:Creator of Omarchy.")?;
    Ok(())
}

// ---- Toasts ---------------------------------------------------------------

#[component]
fn ToastApp() -> impl IntoElement {
    let undone = signal(0_u32);
    let last_id = signal(String::new());
    page()
        .child(om::button("Notify").on_press(move || {
            om::toast(om::Toast::new("Screenshot saved").description("~/Pictures/shot.png"));
        }))
        .child(om::button("Saved").on_press(move || {
            // The api.md example.
            let id = om::toast(
                om::Toast::new("Settings saved")
                    .description("Restart to apply")
                    .success()
                    .action("Undo", move || undone.update(|n| *n += 1)),
            );
            last_id.set(id);
        }))
        .child(om::button("Forget").on_press(move || om::dismiss_toast(&last_id.get_untracked())))
        .child(om::button("Error").danger().on_press(move || {
            om::toast(om::Toast::new("Update failed").error());
        }))
        .child(om::button("Short").on_press(move || {
            om::toast(om::Toast::new("Brief").warning().timeout(Duration::from_secs(1)));
        }))
        .child(om::button("Sticky").on_press(move || {
            om::toast(om::Toast::new("Pinned").info().timeout(None));
        }))
        .child(om::button("Progress").on_press(move || {
            om::toast(om::Toast::new("Step 1").id("progress"));
            om::toast(om::Toast::new("Step 2").id("progress"));
        }))
        .child(text(move || format!("Undone: {}", undone.get())))
        .child(om::toaster())
}

#[test]
fn a_toast_leaves_after_its_timeout() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Notify")?;
    app.press("label:Notify")?;
    app.wait_for("text:Screenshot saved")?;
    app.run_for(Duration::from_secs(3))?;
    assert!(app.screen().shows("Screenshot saved"), "still shown at 3 s");
    app.run_for(Duration::from_secs(4))?;
    app.wait_for_gone("text:Screenshot saved", GONE)?;
    assert_eq!(om::TOAST_TIMEOUT, Duration::from_secs(6));
    Ok(())
}

#[test]
fn an_error_toast_stays_until_dismissed() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Error")?;
    app.press("label:Error")?;
    app.press("label:Sticky")?;
    app.wait_for("text:Update failed")?;
    app.wait_for("text:Pinned")?;
    app.run_for(Duration::from_secs(20))?;
    assert!(app.screen().shows("Update failed"), "an error stays");
    assert!(app.screen().shows("Pinned"), "timeout(None) stays");
    Ok(())
}

#[test]
fn a_toast_sets_its_own_timeout() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Short")?;
    app.press("label:Short")?;
    app.wait_for("text:Brief")?;
    app.run_for(Duration::from_secs(2))?;
    app.wait_for_gone("text:Brief", GONE)?;
    Ok(())
}

#[test]
fn a_toast_action_runs_and_dismisses_it() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Saved")?;
    app.press("label:Saved")?;
    app.wait_for("text:Settings saved")?;
    app.wait_for("text:Restart to apply")?;
    app.press("label:Undo")?;
    app.wait_for("text:Undone: 1")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Settings saved", GONE)?;
    Ok(())
}

#[test]
fn dismiss_toast_removes_it_by_id() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Saved")?;
    app.press("label:Saved")?;
    app.wait_for("text:Settings saved")?;
    app.press("label:Forget")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Settings saved", GONE)?;
    Ok(())
}

#[test]
fn toasting_a_shown_id_replaces_it() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Progress")?;
    app.press("label:Progress")?;
    app.wait_for("text:Step 2")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Step 1", GONE)?;
    Ok(())
}

#[test]
fn escape_dismisses_the_newest_toast() -> Result<(), String> {
    let mut app = start_manual(Root::of::<ToastApp>());
    app.wait_for("label:Error")?;
    app.press("label:Error")?;
    app.wait_for("text:Update failed")?;
    app.press_key("escape")?;
    app.run_for(Duration::from_secs(1))?;
    app.wait_for_gone("text:Update failed", GONE)?;
    Ok(())
}

#[component]
fn EarlyToastApp() -> impl IntoElement {
    // Shown before the toaster mounts: it waits for it.
    om::toast(om::Toast::new("Welcome back"));
    page().child(om::toaster())
}

#[test]
fn a_toast_before_the_toaster_shows_once_it_mounts() -> Result<(), String> {
    let mut app = start(Root::of::<EarlyToastApp>());
    app.wait_for("text:Welcome back")?;
    Ok(())
}

// ---- confirm --------------------------------------------------------------

#[component]
fn ConfirmApp() -> impl IntoElement {
    let last = signal(SharedString::from("none"));
    page()
        .child(om::button("Delete files").on_press(move || {
            spawn(async move {
                // The api.md example.
                let ok = om::confirm(
                    om::ConfirmOptions::new("Delete 3 files?")
                        .action_label("Delete")
                        .danger(),
                )
                .await;
                last.set(if ok { "deleted" } else { "kept" }.into());
            });
        }))
        .child(status(last))
}

#[test]
fn confirm_reports_the_choice() -> Result<(), String> {
    let mut app = start(Root::of::<ConfirmApp>());
    app.wait_for("label:Delete files")?;
    app.on_dialog(|_| DialogAnswer::Press("Delete".to_owned()));
    app.press("label:Delete files")?;
    app.wait_for("text:Last: deleted")?;

    app.on_dialog(|_| DialogAnswer::Press("Cancel".to_owned()));
    app.press("label:Delete files")?;
    app.wait_for("text:Last: kept")?;
    Ok(())
}
