//! The choice controls on their gallery pages: presses and keys change the
//! values the owners keep.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nsh_base::SliderValue;
use nsh_core::{Checked, Color, MountBatch, MountItem, NodeId, ViewStyle, prelude::*};
use nsh_engine::{EngineConfig, Os, Points, Root, Size};
use nsh_omarchy::components as ui;
use nsh_testing::Headless;

/// The gallery on a headless macOS desktop (where Tab stops at controls),
/// opened at the page titled `page`.
fn open(page: &str) -> Result<Headless, String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).map_err(|e| e.to_string())?;
    app.wait_for("Omarchy")?;
    app.press(&format!("label:{page}"))?;
    app.wait_for(&format!("text:{page}"))?;
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
    Ok(())
}

#[test]
fn select_all_is_indeterminate_over_a_partial_choice() -> Result<(), String> {
    let mut app = open("Checkbox")?;
    app.wait_for("text:1 of 3 chosen")?;
    app.press("label:Documents")?;
    app.wait_for("text:2 of 3 chosen")?;
    assert_eq!(
        app.screen()
            .accessibility_state("label:All file types")?
            .checked,
        Some(Checked::Mixed)
    );
    // A press on an indeterminate checkbox asks for checked: all of them.
    app.press("label:All file types")?;
    app.wait_for("text:3 of 3 chosen")?;
    app.press("label:All file types")?;
    app.wait_for("text:0 of 3 chosen")?;
    Ok(())
}

#[test]
fn disabled_checkbox_ignores_presses() -> Result<(), String> {
    let mut app = open("Checkbox")?;
    assert!(!app.screen().enabled("label:Disabled")?);
    let _ = app.press("label:Disabled");
    assert!(!app.screen().enabled("label:Disabled, checked")?);
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
    app.wait_for("text:Appearance: Dark")?;
    app.press("label:System")?;
    app.wait_for("text:Appearance: System")?;
    app.press("label:Roomy")?;
    app.wait_for("text:Density: Roomy")?;
    // The disabled option takes no press.
    assert!(!app.screen().enabled("label:High contrast")?);
    Ok(())
}

#[test]
fn radio_arrow_keys_choose_in_the_gallery() -> Result<(), String> {
    let mut app = open("Radio")?;
    // The page's last stops: the density group, then the appearance group,
    // each entered at its checked radio.
    app.press_key("shift+tab")?;
    app.press_key("shift+tab")?;
    app.press_key("down")?;
    app.wait_for("text:Appearance: System")?;
    // Past the disabled option, around to the first.
    app.press_key("down")?;
    app.wait_for("text:Appearance: Light")?;
    app.press_key("up")?;
    app.wait_for("text:Appearance: System")?;
    // The page did not change under the keys.
    assert!(app.screen().shows("Density: Compact"));
    Ok(())
}

#[test]
fn progress_follows_its_value() -> Result<(), String> {
    let mut app = open("Progress")?;
    app.wait_for("text:Downloading… 35%")?;
    // The page's last stop: the slider driving the bar.
    app.press_key("shift+tab")?;
    app.press_key("end")?;
    app.wait_for("text:Downloading… 100%")?;
    app.press_key("left")?;
    app.wait_for("text:Downloading… 99%")?;
    Ok(())
}

// ---- One control alone, Tab's first stop ------------------------------------

/// `C` alone on a headless macOS desktop.
fn alone<C: Component<Props = ()>>() -> Result<Headless, String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(600.0), Points(400.0)),
        ..EngineConfig::new(Root::of::<C>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).map_err(|e| e.to_string())?;
    app.wait_for("text:/^value /")?;
    Ok(app)
}

#[component]
fn OneSlider() -> impl IntoElement {
    level_slider()
}

fn level_slider() -> View {
    let value = signal(SliderValue::Single(50.0));
    view()
        .w(px(400.0))
        .child(
            ui::slider()
                .accessibility_label("Level")
                .step(5.0)
                .value(move || Some(value.get()))
                .on_change(move |next: &SliderValue| value.set(*next)),
        )
        .child(text(move || format!("value {}", value.get().end())))
}

#[test]
fn slider_keys_step() -> Result<(), String> {
    let mut app = alone::<OneSlider>()?;
    app.wait_for("text:value 50")?;
    app.press_key("tab")?;
    app.press_key("right")?;
    app.wait_for("text:value 55")?;
    app.press_key("left")?;
    app.press_key("left")?;
    app.wait_for("text:value 45")?;
    app.press_key("end")?;
    app.wait_for("text:value 100")?;
    app.press_key("right")?;
    app.wait_for("text:value 100")?;
    app.press_key("home")?;
    app.wait_for("text:value 0")?;
    Ok(())
}

#[component]
fn OneCheckbox() -> impl IntoElement {
    let on = signal(false);
    view()
        .child(
            ui::checkbox("Agree")
                .checked(on)
                .on_change(move |next: &bool| on.set(*next)),
        )
        .child(text(move || format!("value {}", on.get())))
}

#[test]
fn checkbox_space_toggles_and_reports_checked() -> Result<(), String> {
    let mut app = alone::<OneCheckbox>()?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("text:value true")?;
    assert_eq!(
        app.screen().accessibility_state("label:Agree")?.checked,
        Some(Checked::True)
    );
    app.press_key("space")?;
    app.wait_for("text:value false")?;
    Ok(())
}

#[component]
fn OneSwitch() -> impl IntoElement {
    let on = signal(false);
    view()
        .child(
            ui::switch("Wi-Fi")
                .checked(on)
                .on_change(move |next: &bool| on.set(*next)),
        )
        .child(text(move || format!("value {}", on.get())))
}

#[test]
fn switch_space_toggles() -> Result<(), String> {
    let mut app = alone::<OneSwitch>()?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("text:value true")?;
    assert_eq!(
        app.screen().accessibility_state("label:Wi-Fi")?.checked,
        Some(Checked::True)
    );
    Ok(())
}

// ---- What the parts are drawn with -------------------------------------------

/// Every view's last style the platforms got.
type Styles = Arc<Mutex<HashMap<NodeId, ViewStyle>>>;

fn watch_styles(app: &mut Headless) -> Styles {
    let styles = Styles::default();
    let log = styles.clone();
    app.observe(move |_: &str, batch: &MountBatch| {
        let mut log = log.lock().unwrap();
        for item in &batch.items {
            if let MountItem::ViewStyle { id, style } = item {
                log.insert(*id, (**style).clone());
            }
        }
    });
    styles
}

#[component]
fn ThemedSlider() -> impl IntoElement {
    nsh_omarchy::Theme::tokyo_night().apply();
    level_slider()
}

#[test]
fn slider_thumb_classes_resolve_to_the_theme_tokens() -> Result<(), String> {
    let mut app = alone::<ThemedSlider>()?;
    let styles = watch_styles(&mut app);
    let thumb = app
        .screen()
        .press_target("label:Level")?
        .ok_or("no thumb")?;
    app.press_key("tab")?;
    app.wait(std::time::Duration::from_millis(300))?;
    let focused = styles
        .lock()
        .unwrap()
        .get(&thumb)
        .map(|s| s.background_color)
        .ok_or("the thumb was never restyled")?;
    // Focus-visible fills the thumb with Tokyo Night's accent.
    assert_eq!(focused, Color(0x7aa2_f7ff));
    Ok(())
}
