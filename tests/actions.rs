//! The `actions` components, headless: presses, toggled state, keyboard
//! navigation, disabled controls and the gallery pages that show them.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_omarchy::components::{
    ButtonGroupItem, ButtonSize, ButtonVariant, button, button_group, button_primary, icon_button,
    icon_toggle, link, tab_list, toggle, toggle_group,
};
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
    crate_init();
    let count = signal(0_u32);
    let add = move |by: u32| move || count.update(|n| *n += by);
    view()
        .child(button_primary("Apply").on_press(add(1)))
        .child(button("Unavailable").disabled(true).on_press(add(100)))
        .child(
            icon_button(text("+"), "Add panel", ButtonVariant::Ghost, ButtonSize::Sm)
                .on_press(add(10)),
        )
        .child(text(move || format!("count={}", count.get())))
}

fn crate_init() {
    nsh_omarchy::init();
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
    // The press finds nothing to press, or presses nothing.
    let _ = app.press("Unavailable");
    app.press("Apply")?;
    app.wait_for("count=1")?;
    Ok(())
}

#[test]
fn the_keyboard_reaches_and_presses_buttons() -> Result<(), String> {
    let mut app = desktop(Root::of::<Buttons>());
    app.wait_for("count=0")?;
    // Tab: Apply; Space presses it.
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("count=1")?;
    // Tab skips the disabled button: Add panel; Return presses it.
    app.press_key("tab")?;
    app.press_key("enter")?;
    app.wait_for("count=11")?;
    Ok(())
}

// ---- Toggle -----------------------------------------------------------------

#[component]
fn Toggles() -> impl IntoElement {
    crate_init();
    let favorite = signal(false);
    let bold = signal(false);
    let locked = signal(0_u32);
    view()
        .child(
            toggle("Favorite")
                .pressed(favorite)
                .on_change(move |on: &bool| favorite.set(*on)),
        )
        .child(
            toggle("Locked")
                .disabled(true)
                .on_change(move |_: &bool| locked.update(|n| *n += 1)),
        )
        .child(toggle_group(
            "Formatting",
            [icon_toggle(text("B"), "Bold", ButtonSize::Md)
                .pressed(bold)
                .on_change(move |on: &bool| bold.set(*on))],
        ))
        .child(text(move || {
            format!(
                "favorite={} bold={} locked={}",
                favorite.get(),
                bold.get(),
                locked.get()
            )
        }))
}

#[test]
fn a_toggle_turns_on_and_off() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false bold=false locked=0")?;
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Favorite]")?
            .pressed,
        Some(nsh_core::Checked::False)
    );
    app.press("Favorite")?;
    app.wait_for("favorite=true bold=false locked=0")?;
    assert_eq!(
        app.screen()
            .accessibility_state("role:button[Favorite]")?
            .pressed,
        Some(nsh_core::Checked::True)
    );
    app.press("label:Bold")?;
    app.wait_for("favorite=true bold=true locked=0")?;
    app.press("Favorite")?;
    app.wait_for("favorite=false bold=true locked=0")?;
    Ok(())
}

#[test]
fn a_disabled_toggle_stays_put() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false bold=false locked=0")?;
    assert!(!app.screen().enabled("Locked")?);
    let _ = app.press("Locked");
    app.press("Favorite")?;
    app.wait_for("favorite=true bold=false locked=0")?;
    Ok(())
}

#[test]
fn space_toggles_the_focused_toggle() -> Result<(), String> {
    let mut app = desktop(Root::of::<Toggles>());
    app.wait_for("favorite=false bold=false locked=0")?;
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("favorite=true bold=false locked=0")?;
    // The disabled toggle is no stop: Tab goes on to the group.
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("favorite=true bold=true locked=0")?;
    Ok(())
}

// ---- Button group -------------------------------------------------------------

#[component]
fn Groups() -> impl IntoElement {
    crate_init();
    let position = signal(Some(0_usize));
    let changes = signal(0_u32);
    let after = signal(0_u32);
    view()
        .child(button("Before"))
        .child(button_group(
            "Position",
            [
                ButtonGroupItem::new("Top"),
                "Right".into(),
                ButtonGroupItem::new("Bottom").disabled(true),
                "Left".into(),
            ],
            position,
            move |index| {
                changes.update(|n| *n += 1);
                position.set(Some(index));
            },
        ))
        .child(button("After").on_press(move || after.update(|n| *n += 1)))
        .child(text(move || {
            format!(
                "position={} changes={} after={}",
                position.get().map_or(-1, |i| i as i64),
                changes.get(),
                after.get()
            )
        }))
}

#[test]
fn a_segment_press_chooses_it_once() -> Result<(), String> {
    let mut app = desktop(Root::of::<Groups>());
    app.wait_for("position=0 changes=0 after=0")?;
    assert!(app.screen().matches("label:Position")?);
    app.press("Left")?;
    app.wait_for("position=3 changes=1 after=0")?;
    // The chosen segment again: nothing to report.
    app.press("Left")?;
    app.press("Top")?;
    app.wait_for("position=0 changes=2 after=0")?;
    assert!(!app.screen().enabled("Bottom")?);
    let _ = app.press("Bottom");
    app.press("Right")?;
    app.wait_for("position=1 changes=3 after=0")?;
    Ok(())
}

#[test]
fn arrows_move_between_segments_and_space_chooses() -> Result<(), String> {
    let mut app = desktop(Root::of::<Groups>());
    app.wait_for("position=0 changes=0 after=0")?;
    app.press_key("tab")?; // Before
    app.press_key("tab")?; // the group, at the chosen segment (Top)
    app.press_key("right")?; // Right: focus only
    app.wait_for("position=0 changes=0 after=0")?;
    app.press_key("right")?; // past the disabled Bottom: Left
    app.press_key("space")?;
    app.wait_for("position=3 changes=1 after=0")?;
    app.press_key("home")?; // Top
    app.press_key("enter")?;
    app.wait_for("position=0 changes=2 after=0")?;
    // The group is one stop: Tab leaves it for After.
    app.press_key("tab")?;
    app.press_key("space")?;
    app.wait_for("position=0 changes=2 after=1")?;
    Ok(())
}

// ---- Tab list -----------------------------------------------------------------

#[component]
fn Tabs() -> impl IntoElement {
    crate_init();
    let tab = signal(Some(0_usize));
    view()
        .child(tab_list(
            "Sections",
            [
                ButtonGroupItem::new("One"),
                "Two".into(),
                ButtonGroupItem::new("Three").disabled(true),
                "Four".into(),
            ],
            tab,
            move |index| tab.set(Some(index)),
        ))
        .child(text(move || {
            format!("tab={}", tab.get().map_or(-1, |i| i as i64))
        }))
}

#[test]
fn a_tab_press_selects_it() -> Result<(), String> {
    let mut app = desktop(Root::of::<Tabs>());
    app.wait_for("tab=0")?;
    assert!(app.screen().selected("role:tab[One]")?);
    app.press("Four")?;
    app.wait_for("tab=3")?;
    assert!(app.screen().selected("role:tab[Four]")?);
    assert!(!app.screen().selected("role:tab[One]")?);
    assert!(!app.screen().enabled("Three")?);
    Ok(())
}

#[test]
fn arrows_select_tabs_past_disabled_ones() -> Result<(), String> {
    let mut app = desktop(Root::of::<Tabs>());
    app.wait_for("tab=0")?;
    app.press_key("tab")?;
    app.press_key("right")?;
    app.wait_for("tab=1")?;
    app.press_key("right")?;
    app.wait_for("tab=3")?;
    app.press_key("home")?;
    app.wait_for("tab=0")?;
    app.press_key("left")?;
    app.wait_for("tab=3")?;
    Ok(())
}

// ---- Link ---------------------------------------------------------------------

#[component]
fn Links() -> impl IntoElement {
    crate_init();
    let opened = signal(SharedString::from("none"));
    let dead = signal(0_u32);
    view()
        .child(link("Manual", "https://omarchy.org/manual"))
        .child(
            link("Release notes", "/releases")
                .open_with(nsh_base::LinkOpenWith::App)
                .on_open(move |href: &SharedString| opened.set(href.clone())),
        )
        .child(
            link("Unavailable", "https://omarchy.org")
                .disabled(true)
                .on_press(move || dead.update(|n| *n += 1)),
        )
        .child(text(move || {
            format!("opened={} dead={}", opened.get(), dead.get())
        }))
}

#[test]
fn a_link_opens_its_href() -> Result<(), String> {
    let mut app = desktop(Root::of::<Links>());
    app.wait_for("opened=none dead=0")?;
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
    app.wait_for("opened=/releases dead=0")?;
    Ok(())
}

#[test]
fn a_disabled_link_goes_nowhere() -> Result<(), String> {
    let mut app = desktop(Root::of::<Links>());
    app.wait_for("opened=none dead=0")?;
    assert!(!app.screen().enabled("Unavailable")?);
    let _ = app.press("Unavailable");
    // Tab: Manual, Release notes; the disabled link is no stop, so the
    // third Tab cannot open it.
    app.press_key("tab")?;
    app.press_key("tab")?;
    app.press_key("enter")?;
    app.wait_for("opened=/releases dead=0")?;
    assert!(
        !app.screen()
            .opened_urls()
            .iter()
            .any(|url| url == "https://omarchy.org")
    );
    Ok(())
}

// ---- Gallery pages --------------------------------------------------------------

fn gallery() -> Headless {
    let mut app = desktop(Root::of::<nsh_omarchy::gallery::Gallery>());
    app.wait_for("Omarchy").unwrap();
    app
}

#[test]
fn the_button_page_counts_activations() -> Result<(), String> {
    let mut app = gallery();
    app.press("label:Button")?;
    app.wait_for("text:Activations: 0")?;
    app.press("text:Apply >> nth=0")?;
    app.press("text:Apply >> nth=1")?;
    app.press("label:Add panel")?;
    app.wait_for("text:Activations: 3")?;
    app.press("Pin")?;
    app.wait_for("text:Pinned: on")?;
    app.press("Reset")?;
    app.wait_for("text:Activations: 0")?;
    Ok(())
}

#[test]
fn the_toggle_pages_follow_their_toggles() -> Result<(), String> {
    let mut app = gallery();
    app.press("label:Toggle")?;
    app.wait_for("text:Favorite panel: off")?;
    app.press("Favorite panel")?;
    app.wait_for("text:Favorite panel: on")?;

    app.press("label:Toggle group")?;
    app.wait_for("text:4 articles")?;
    app.press("Draft")?;
    app.wait_for("text:2 articles")?;
    assert!(!app.shows("Autumn release notes"));
    app.press("In review")?;
    app.press("Published")?;
    app.wait_for("No articles match.")?;
    app.press("label:Italic")?;
    app.wait_for("text:Bold off · Italic on · Underline off")?;
    Ok(())
}

#[test]
fn the_button_group_page_follows_its_choices() -> Result<(), String> {
    let mut app = gallery();
    app.press("label:Button group")?;
    app.wait_for("text:Toolbar is placed at the top.")?;
    app.press("Bottom")?;
    app.wait_for("text:Toolbar is placed at the bottom.")?;
    app.wait_for("Overview panel")?;
    app.press("Activity")?;
    app.wait_for("Activity panel")?;
    Ok(())
}

#[test]
fn the_link_page_reports_an_app_link() -> Result<(), String> {
    let mut app = gallery();
    app.press("label:Link")?;
    app.wait_for("text:Opened: nothing yet")?;
    app.press("Release notes")?;
    app.wait_for("text:Opened: /releases")?;
    Ok(())
}
