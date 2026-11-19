//! The choices, headless through the public API: presses and keys change
//! the values, owned by the app or kept by the control.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nsh_core::{Checked, Color, MountBatch, MountItem, NodeId, ViewStyle, prelude::*};
use nsh_engine::{EngineConfig, Os, Points, Root, Size};
use nsh_omarchy::{self as om, SliderValue};
use nsh_testing::Headless;

/// `C` on a headless macOS desktop (where Tab stops at controls), once it
/// shows `ready`.
fn open<C: Component<Props = ()>>(ready: &str) -> Result<Headless, String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(800.0), Points(600.0)),
        ..EngineConfig::new(Root::of::<C>())
    };
    config.environment.os = Os::MacOs;
    let mut app = Headless::start(config).map_err(|e| e.to_string())?;
    app.wait_for(ready)?;
    Ok(app)
}

fn on_off(on: bool) -> &'static str {
    if on { "on" } else { "off" }
}

// ---- Checkbox -------------------------------------------------------------------

#[component]
fn Checkboxes() -> impl IntoElement {
    om::init();
    let remember = signal(false);
    // A "select all" over three items: indeterminate while some are chosen.
    let items = signal([true, false, false]);
    let chosen = computed(move || items.get().iter().filter(|on| **on).count());
    let item = move |index: usize, label: &'static str| {
        om::checkbox(label)
            .checked(move || items.get()[index])
            .on_change(move |next: &bool| items.update(|all| all[index] = *next))
    };
    let kept = signal(0_u32);
    view()
        // The api.md example.
        .child(
            om::checkbox("Remember me")
                .checked(move || remember.get())
                .on_change(move |on: &bool| remember.set(*on)),
        )
        .child(
            om::checkbox("All file types")
                .checked(move || chosen.get() == 3)
                .indeterminate(move || (1..3).contains(&chosen.get()))
                .on_change(move |next: &bool| items.set([*next; 3])),
        )
        .child(item(0, "Images"))
        .child(item(1, "Documents"))
        .child(item(2, "Archives"))
        .child(om::checkbox("Disabled").disabled(true))
        // Uncontrolled.
        .child(om::checkbox("Kept").on_change(move |_: &bool| kept.update(|n| *n += 1)))
        .child(text(move || {
            format!(
                "remember {} · {} of 3 · kept {}",
                on_off(remember.get()),
                chosen.get(),
                kept.get()
            )
        }))
}

fn checked(app: &Headless, selector: &str) -> Result<Option<Checked>, String> {
    Ok(app.screen().accessibility_state(selector)?.checked)
}

#[test]
fn a_checkbox_checks() -> Result<(), String> {
    let mut app = open::<Checkboxes>("text:/remember off · 1 of 3 · kept 0/")?;
    app.press("label:Remember me")?;
    app.wait_for("text:/remember on/")?;
    assert_eq!(checked(&app, "label:Remember me")?, Some(Checked::True));
    app.press("label:Remember me")?;
    app.wait_for("text:/remember off/")?;
    assert!(!app.screen().enabled("label:Disabled")?);
    Ok(())
}

#[test]
fn select_all_is_indeterminate_over_a_partial_choice() -> Result<(), String> {
    let mut app = open::<Checkboxes>("text:/1 of 3/")?;
    assert_eq!(checked(&app, "label:All file types")?, Some(Checked::Mixed));
    app.press("label:All file types")?;
    app.wait_for("text:/3 of 3/")?;
    app.press("label:All file types")?;
    app.wait_for("text:/0 of 3/")?;
    Ok(())
}

#[test]
fn an_uncontrolled_checkbox_keeps_its_value() -> Result<(), String> {
    let mut app = open::<Checkboxes>("text:/kept 0/")?;
    app.press("label:Kept")?;
    app.wait_for("text:/kept 1/")?;
    assert_eq!(checked(&app, "label:Kept")?, Some(Checked::True));
    app.press("label:Kept")?;
    app.wait_for("text:/kept 2/")?;
    assert_eq!(checked(&app, "label:Kept")?, Some(Checked::False));
    Ok(())
}

#[test]
fn space_checks_the_focused_checkbox() -> Result<(), String> {
    let mut app = open::<Checkboxes>("text:/remember off/")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("text:/remember on/")?;
    Ok(())
}

// ---- Switch ---------------------------------------------------------------------

#[component]
fn Switches() -> impl IntoElement {
    om::init();
    let wifi = signal(false);
    view()
        .child(
            om::switch("Wi-Fi")
                .checked(move || wifi.get())
                .on_change(move |on: &bool| wifi.set(*on)),
        )
        .child(om::switch("Bluetooth"))
        .child(om::switch("Airplane mode").disabled(true))
        .child(text(move || format!("Wi-Fi {}", on_off(wifi.get()))))
}

#[test]
fn a_switch_switches() -> Result<(), String> {
    let mut app = open::<Switches>("text:/Wi-Fi off/")?;
    app.press("label:Wi-Fi")?;
    app.wait_for("text:/Wi-Fi on/")?;
    assert_eq!(checked(&app, "label:Wi-Fi")?, Some(Checked::True));
    // Uncontrolled: it keeps its own value.
    app.press("label:Bluetooth")?;
    app.wait_until(
        "Bluetooth on",
        std::time::Duration::from_secs(2),
        |screen| {
            screen
                .accessibility_state("label:Bluetooth")
                .is_ok_and(|s| s.checked == Some(Checked::True))
        },
    )?;
    assert!(!app.screen().enabled("label:Airplane mode")?);
    // Space switches the focused one.
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("text:/Wi-Fi off/")?;
    Ok(())
}

// ---- RadioGroup -----------------------------------------------------------------

#[component]
fn Radios() -> impl IntoElement {
    om::init();
    let plan = signal(Some(SharedString::from("pro")));
    let density = signal(SharedString::from("none"));
    view()
        .child(
            om::radio_group()
                .accessibility_label("Plan")
                .value(move || plan.get())
                .on_change(move |next: &SharedString| plan.set(Some(next.clone())))
                .child(om::radio("free", "Free"))
                .child(om::radio("pro", "Pro"))
                .child(om::radio("team", "Team"))
                .child(om::radio("enterprise", "Enterprise").disabled(true)),
        )
        .child(
            // Uncontrolled, in a row.
            om::radio_group()
                .horizontal()
                .on_change(move |next: &SharedString| density.set(next.clone()))
                .child(om::radio("compact", "Compact"))
                .child(om::radio("roomy", "Roomy")),
        )
        .child(text(move || {
            format!(
                "plan {} · density {}",
                plan.get().unwrap_or_default(),
                density.get()
            )
        }))
}

#[test]
fn a_radio_press_chooses_it() -> Result<(), String> {
    let mut app = open::<Radios>("text:/plan pro · density none/")?;
    assert_eq!(checked(&app, "label:Pro")?, Some(Checked::True));
    app.press("label:Team")?;
    app.wait_for("text:/plan team/")?;
    assert_eq!(checked(&app, "label:Team")?, Some(Checked::True));
    assert_eq!(checked(&app, "label:Pro")?, Some(Checked::False));
    assert!(!app.screen().enabled("label:Enterprise")?);
    app.press("label:Roomy")?;
    app.wait_for("text:/density roomy/")?;
    assert_eq!(checked(&app, "label:Roomy")?, Some(Checked::True));
    Ok(())
}

#[test]
fn arrows_choose_radios() -> Result<(), String> {
    let mut app = open::<Radios>("text:/plan pro/")?;
    app.press_key("tab")?; // the plan group, at Pro
    app.press_key("down")?;
    app.wait_for("text:/plan team/")?;
    // Past the disabled option, around to the first.
    app.press_key("down")?;
    app.wait_for("text:/plan free/")?;
    app.press_key("up")?;
    app.wait_for("text:/plan team/")?;
    Ok(())
}

// ---- Slider ---------------------------------------------------------------------

#[component]
fn Sliders() -> impl IntoElement {
    om::init();
    let level = signal(SliderValue::Single(50.0));
    let ends = signal(0_u32);
    view()
        .w(px(400.0))
        .child(
            om::slider()
                .accessibility_label("Level")
                .step(5.0)
                .show_value(true)
                .value(move || level.get())
                .on_change(move |next: &SliderValue| level.set(*next))
                .on_change_end(move |_: &SliderValue| ends.update(|n| *n += 1)),
        )
        .child(om::slider().accessibility_label("Kept").min(10.0).show_value(true))
        .child(text(move || format!("value {}", level.get().end())))
}

#[test]
fn slider_keys_step() -> Result<(), String> {
    let mut app = open::<Sliders>("text:/value 50/")?;
    app.press_key("tab")?;
    app.press_key("right")?;
    app.wait_for("text:/value 55/")?;
    app.press_key("left")?;
    app.press_key("left")?;
    app.wait_for("text:/value 45/")?;
    app.press_key("end")?;
    app.wait_for("text:/value 100/")?;
    app.press_key("home")?;
    app.wait_for("text:/value 0/")?;
    Ok(())
}

#[test]
fn an_uncontrolled_slider_starts_at_min_and_keeps_its_value() -> Result<(), String> {
    let mut app = open::<Sliders>("text:/value 50/")?;
    app.press_key("tab")?;
    app.press_key("tab")?; // Kept
    app.wait_for("text:10")?;
    app.press_key("right")?;
    app.wait_for("text:11")?;
    Ok(())
}

// ---- Progress -------------------------------------------------------------------

#[component]
fn Bars() -> impl IntoElement {
    om::init();
    view()
        .w(px(400.0))
        .child(om::progress().value(64.0).label("Downloading").show_value(true))
        .child(om::progress().value(92.0).label("Disk").warning().show_value(true))
        .child(om::progress().indeterminate(true).label("Installing").show_value(true))
}

#[test]
fn progress_shows_its_label_and_value() -> Result<(), String> {
    let mut app = open::<Bars>("text:Downloading")?;
    app.wait_for("text:64%")?;
    app.wait_for("text:92%")?;
    app.wait_for("text:Installing")?;
    // Indeterminate: no percentage.
    assert!(!app.shows("0%"));
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
    om::Theme::tokyo_night().apply();
    let level = signal(SliderValue::Single(50.0));
    view()
        .w(px(400.0))
        .child(
            om::slider()
                .accessibility_label("Level")
                .value(move || level.get())
                .on_change(move |next: &SliderValue| level.set(*next)),
        )
        .child(text(move || format!("value {}", level.get().end())))
}

#[test]
fn slider_thumb_classes_resolve_to_the_theme_tokens() -> Result<(), String> {
    let mut app = open::<ThemedSlider>("text:/value 50/")?;
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
