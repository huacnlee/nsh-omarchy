//! The actions, headless through the public API: presses, toggled state,
//! groups, keyboard navigation, disabled controls and links.

use nsh_core::{Checked, prelude::*};
use nsh_engine::{EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_omarchy::{self as om, IconName, LinkTarget};
use nsh_testing::Headless;

/// A desktop (macOS), where Tab moves the keyboard focus between controls.
fn desktop(root: Root) -> Headless {
    let environment = Environment {
        os: Os::MacOs,
        idiom: Idiom::Desktop,
        ..Environment::default()
    };
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        environment,
        ..EngineConfig::new(root)
    })
    .unwrap()
}

// ---- Button -----------------------------------------------------------------

#[component]
fn Buttons() -> impl IntoElement {
    om::init();
    let count = signal(0_u32);
    let add = move |by: u32| move || count.update(|n| *n += by);
    view()
        .child(om::button("Apply").primary().small().on_press(add(1)))
        .child(om::button("Unavailable").disabled(true).on_press(add(100)))
        .child(om::icon_button(IconName::Plus, "Add panel").small().on_press(add(10)))
        .child(text(move || format!("count={}", count.get())))
}

#[test]
fn a_button_press_runs_its_action() -> Result<(), String> {
    let mut app = desktop(Root::of::<Buttons>());
    app.wait_for("count=0")?;
    app.press("Apply")?;
    app.wait_for("count=1")?;
    app.press("label:Add panel")?;
    app.wait_for("count=11")?;
    Ok(())
}

#[test]
fn a_disabled_button_takes_no_press() -> Result<(), String> {
    let mut app = desktop(Root::of::<Buttons>());
    app.wait_for("count=0")?;
    assert!(!app.screen().enabled("Unavailable")?);
    assert!(app.screen().enabled("Apply")?);
    let _ = app.press("Unavailable");
    app.press("Apply")?;
    app.wait_for("count=1")?;
    Ok(())
}

#[test]
fn the_keyboard_reaches_and_presses_buttons() -> Result<(), String> {
    let mut app = desktop(Root::of::<Buttons>());
    app.wait_for("count=0")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("count=1")?;
    // Tab skips the disabled button.
    app.press_key("tab")?;
    app.press_key("enter")?;
    app.wait_for("count=11")?;
    Ok(())
}

// ---- ButtonGroup ----------------------------------------------------------------

#[component]
fn Joined() -> impl IntoElement {
    om::init();
    let last = signal(SharedString::from("none"));
    let press = move |name: &'static str| move || last.set(name.into());
    view()
        .child(
            om::button_group()
                .small()
                .child(om::button("Copy").on_press(press("copy")))
                .child(om::button("Paste").on_press(press("paste"))),
        )
        .child(
            om::button_group()
                .disabled(true)
                .child(om::button("Undo").on_press(press("undo"))),
        )
        .child(text(move || format!("last={}", last.get())))
}

#[test]
fn a_button_group_passes_presses_and_disables_its_buttons() -> Result<(), String> {
    let mut app = desktop(Root::of::<Joined>());
    app.wait_for("last=none")?;
    app.press("Paste")?;
    app.wait_for("last=paste")?;
    assert!(!app.screen().enabled("Undo")?);
    let _ = app.press("Undo");
    app.press("Copy")?;
    app.wait_for("last=copy")?;
    // Each button is its own stop of Tab.
    app.press_key("tab")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("last=paste")?;
    Ok(())
}

// ---- Toggle -----------------------------------------------------------------

#[component]
fn Toggles() -> impl IntoElement {
    om::init();
    let favorite = signal(false);
    let locked = signal(0_u32);
    let wrap_changes = signal(0_u32);
    view()
        .child(
            om::toggle("Favorite")
                .icon(IconName::Star)
                .checked(move || favorite.get())
                .on_change(move |on: &bool| favorite.set(*on)),
        )
        .child(
            om::toggle("Locked")
                .disabled(true)
                .on_change(move |_: &bool| locked.update(|n| *n += 1)),
        )
        // Uncontrolled: it keeps its own value.
        .child(om::toggle("Wrap").small().on_change(move |_: &bool| wrap_changes.update(|n| *n += 1)))
        .child(text(move || {
            format!(
                "favorite={} locked={} wraps={}",
                favorite.get(),
                locked.get(),
                wrap_changes.get()
            )
        }))
}

fn pressed(app: &Headless, selector: &str) -> Result<Option<Checked>, String> {
    Ok(app.screen().accessibility_state(selector)?.pressed)
}

#[test]
fn a_toggle_turns_on_and_off() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false locked=0 wraps=0")?;
    assert_eq!(pressed(&app, "role:button[Favorite]")?, Some(Checked::False));
    app.press("role:button[Favorite]")?;
    app.wait_for("favorite=true locked=0 wraps=0")?;
    assert_eq!(pressed(&app, "role:button[Favorite]")?, Some(Checked::True));
    app.press("role:button[Favorite]")?;
    app.wait_for("favorite=false locked=0 wraps=0")?;
    Ok(())
}

#[test]
fn an_uncontrolled_toggle_keeps_its_value() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("text:/wraps=0/")?;
    app.press("role:button[Wrap]")?;
    app.wait_for("text:/wraps=1/")?;
    assert_eq!(pressed(&app, "role:button[Wrap]")?, Some(Checked::True));
    app.press("role:button[Wrap]")?;
    app.wait_for("text:/wraps=2/")?;
    assert_eq!(pressed(&app, "role:button[Wrap]")?, Some(Checked::False));
    Ok(())
}

#[test]
fn a_disabled_toggle_stays_put() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false locked=0 wraps=0")?;
    assert!(!app.screen().enabled("role:button[Locked]")?);
    let _ = app.press("role:button[Locked]");
    app.press("role:button[Favorite]")?;
    app.wait_for("favorite=true locked=0 wraps=0")?;
    Ok(())
}

#[test]
fn space_toggles_the_focused_toggle() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false locked=0 wraps=0")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("favorite=true locked=0 wraps=0")?;
    // The disabled toggle is no stop.
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("favorite=true locked=0 wraps=1")?;
    Ok(())
}

// ---- ToggleGroup ----------------------------------------------------------------

#[component]
fn Groups() -> impl IntoElement {
    om::init();
    let marks = signal(vec![SharedString::from("italic")]);
    let align = signal(Vec::<SharedString>::new());
    let after = signal(0_u32);
    view()
        .child(om::button("Before"))
        .child(
            om::toggle_group()
                .value(move || marks.get())
                .on_change(move |next: &Vec<SharedString>| marks.set(next.clone()))
                .child(om::toggle("Bold").value("bold"))
                .child(om::toggle("Italic").value("italic"))
                .child(om::toggle("Strike").value("strike").disabled(true))
                .child(om::toggle("Underline").value("underline")),
        )
        .child(
            // Uncontrolled and single: the group keeps one value.
            om::toggle_group()
                .single(true)
                .on_change(move |next: &Vec<SharedString>| align.set(next.clone()))
                .child(om::toggle("Left").value("left"))
                .child(om::toggle("Right").value("right")),
        )
        .child(om::button("After").on_press(move || after.update(|n| *n += 1)))
        .child(text(move || {
            format!(
                "marks={} align={} after={}",
                marks.get().join(","),
                align.get().join(","),
                after.get()
            )
        }))
}

#[test]
fn a_toggle_group_reports_its_values() -> Result<(), String> {
    let mut app = desktop(Root::of::<Groups>());
    app.wait_for("marks=italic align= after=0")?;
    assert_eq!(pressed(&app, "role:button[Italic]")?, Some(Checked::True));
    app.press("role:button[Bold]")?;
    app.wait_for("marks=italic,bold align= after=0")?;
    app.press("role:button[Italic]")?;
    app.wait_for("marks=bold align= after=0")?;
    assert_eq!(pressed(&app, "role:button[Italic]")?, Some(Checked::False));
    assert!(!app.screen().enabled("role:button[Strike]")?);
    Ok(())
}

#[test]
fn a_single_group_keeps_one_value() -> Result<(), String> {
    let mut app = desktop(Root::of::<Groups>());
    app.wait_for("marks=italic align= after=0")?;
    app.press("role:button[Left]")?;
    app.wait_for("text:/align=left /")?;
    app.press("role:button[Right]")?;
    app.wait_for("text:/align=right /")?;
    assert_eq!(pressed(&app, "role:button[Left]")?, Some(Checked::False));
    assert_eq!(pressed(&app, "role:button[Right]")?, Some(Checked::True));
    Ok(())
}

#[test]
fn arrows_move_inside_a_toggle_group() -> Result<(), String> {
    let mut app = desktop(Root::of::<Groups>());
    app.wait_for("marks=italic align= after=0")?;
    app.press_key("tab")?; // Before
    app.press_key("tab")?; // the group, at its pressed toggle (Italic)
    app.press_key("right")?; // past the disabled Strike: Underline, focus only
    app.wait_for("marks=italic align= after=0")?;
    app.press_key("space")?;
    app.wait_for("marks=italic,underline align= after=0")?;
    // Each group is one stop: Tab goes on to the next group, then After.
    app.press_key("tab")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("text:/after=1/")?;
    Ok(())
}

// ---- Link ---------------------------------------------------------------------

#[component]
fn Links() -> impl IntoElement {
    om::init();
    let opened = signal(SharedString::from("none"));
    view()
        .child(om::link("Manual", "https://omarchy.org/manual"))
        .child(
            om::link("Release notes", "/releases")
                .open_with(LinkTarget::App)
                .on_press(move || opened.set("/releases".into())),
        )
        .child(text(move || format!("opened={}", opened.get())))
}

#[test]
fn a_link_opens_its_href() -> Result<(), String> {
    let mut app = desktop(Root::of::<Links>());
    app.wait_for("opened=none")?;
    app.press("Manual")?;
    app.wait_until(
        "the manual opened",
        std::time::Duration::from_secs(2),
        |screen| {
            screen
                .opened_urls()
                .iter()
                .any(|url| url == "https://omarchy.org/manual")
        },
    )?;
    app.press("Release notes")?;
    app.wait_for("opened=/releases")?;
    assert!(!app.screen().opened_urls().iter().any(|url| url == "/releases"));
    assert!(app.screen().matches("role:link[Manual]")?);
    Ok(())
}

// ---- The api.md examples ----------------------------------------------------------

#[component]
fn Example() -> impl IntoElement {
    om::init();
    let saved = signal(0_u32);
    let save = move || saved.update(|n| *n += 1);
    view()
        .child(om::button("Save").primary().small().on_press(move || save()))
        .child(text(move || format!("saved={}", saved.get())))
}

#[test]
fn the_api_example_compiles_and_runs() -> Result<(), String> {
    let mut app = desktop(Root::of::<Example>());
    app.wait_for("saved=0")?;
    app.press("Save")?;
    app.wait_for("saved=1")?;
    Ok(())
}
