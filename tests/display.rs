//! Display through the public API: every constructor of api.md's Display
//! table builds and draws, and the interactive ones respond.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_omarchy::{self as om, IconName};
use nsh_testing::Headless;

fn start(root: Root) -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1000.0), Points(1400.0)),
        ..EngineConfig::new(root)
    })
    .unwrap()
}

fn expect_texts(app: &mut Headless, texts: &[&str]) -> Result<(), String> {
    for t in texts {
        app.wait_for(&format!("text:{t}"))?;
    }
    Ok(())
}

/// Every constructor of the table, with its builders and shorthands.
#[component]
fn Showcase() -> impl IntoElement {
    om::init();
    view()
        .gap(px(8.0))
        .child(
            om::card()
                .title("Updates")
                .description("3 packages can be upgraded.")
                .footer(om::button("Update").primary().small())
                .child(om::caption("Last checked 5 minutes ago.")),
        )
        .child(om::panel("Workspace").child(om::body("Raised panel")))
        .child(om::panel("Outlined").outline().child(om::separator()))
        .child(om::panel("Filled").fill())
        .child(
            view()
                .flex_row()
                .child(om::body("Files"))
                .child(om::separator().vertical())
                .child(om::body("Edit")),
        )
        .child(om::badge("Neutral"))
        .child(om::badge("New").primary())
        .child(om::badge("Online").success().outline(true))
        .child(om::badge("Beta").warning().size(om::ControlSize::Small))
        .child(om::badge("Failed").danger().variant(om::BadgeVariant::Danger))
        .child(om::keycap("esc"))
        .child(om::keycap("mod"))
        .child(om::shortcut("mod-shift-p"))
        .child(om::avatar("Grace Hopper").size(om::ControlSize::Large))
        .child(
            om::empty_state("No windows")
                .description("Open an app with super-space.")
                .icon(IconName::Inbox)
                .child(om::button("Open launcher")),
        )
        .child(om::icon(IconName::Lock).label("Encrypted").size(24.0))
        .child(om::icon(IconName::Star))
        .child(om::heading("Heading"))
        .child(om::title("Title"))
        .child(om::body("Body"))
        .child(om::caption("Caption"))
        .child(om::code("omarchy-update"))
        .child(om::markdown("Some **bold** words").selectable(false))
        // The text this platform shows for a chord.
        .child(text(format!(
            "Save: {}, {}",
            om::shortcut_label("mod-s"),
            om::shortcut_label("Ctrl+Shift+P")
        )))
}

#[test]
fn every_constructor_draws() -> Result<(), String> {
    let mut app = start(Root::of::<Showcase>());
    expect_texts(
        &mut app,
        &[
            "Updates",
            "3 packages can be upgraded.",
            "Last checked 5 minutes ago.",
            "Update",
            "Workspace",
            "Raised panel",
            "Outlined",
            "Files",
            "Edit",
            "NEUTRAL",
            "NEW",
            "ONLINE",
            "BETA",
            "FAILED",
            "esc",
            "GH",
            "No windows",
            "Open an app with super-space.",
            "Open launcher",
            "Heading",
            "Title",
            "Body",
            "Caption",
            "omarchy-update",
            "Some bold words",
        ],
    )?;
    app.wait_for("label:Encrypted")?;
    app.wait_for("label:Grace Hopper")?;
    // `mod` reads as this platform's command key, in a keycap and a chord.
    let command = if app.shows("cmd") { "cmd" } else { "ctrl" };
    app.wait_for(&format!("text:{command}"))?;
    app.wait_for(&format!("label:{command} shift p"))?;
    app.wait_for(&format!("text:Save: {command}-s, ctrl-shift-p"))?;
    assert!(app.screen().count("role:keyboardkey")? >= 5);
    Ok(())
}

/// Live props: the type ramp and a badge follow their signals.
#[component]
fn Live() -> impl IntoElement {
    om::init();
    let count = signal(1_u32);
    view()
        .child(om::heading(move || format!("{} windows", count.get())))
        .child(om::badge(move || format!("{} new", count.get())).primary())
        .child(om::button("More").on_press(move || count.update(|n| *n += 1)))
}

#[test]
fn text_follows_its_signal() -> Result<(), String> {
    let mut app = start(Root::of::<Live>());
    expect_texts(&mut app, &["1 windows", "1 NEW"])?;
    app.press("label:More")?;
    expect_texts(&mut app, &["2 windows", "2 NEW"])
}

/// A Markdown document reporting its links.
#[component]
fn Notes() -> impl IntoElement {
    om::init();
    let pressed = signal(String::new());
    view()
        .child(
            om::markdown("# Release notes\n\nRead the [docs](https://omarchy.org).")
                .on_link_press(move |url: &String| pressed.set(url.clone())),
        )
        .child(text(move || format!("Link pressed: {}", pressed.get())))
}

#[test]
fn markdown_reports_link_presses() -> Result<(), String> {
    let mut app = start(Root::of::<Notes>());
    expect_texts(&mut app, &["Release notes", "Link pressed: "])?;
    app.press("text:docs")?;
    app.wait_for("text:Link pressed: https://omarchy.org")
}

#[test]
fn icon_markup_is_well_formed() {
    let mut names = std::collections::HashSet::new();
    for name in IconName::ALL {
        let svg = name.svg_markup();
        assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"), "{}", name.name());
        assert!(svg.contains("currentColor"));
        assert!(nsh_core::svg::drawing(&svg).is_ok(), "{} does not parse", name.name());
        assert!(names.insert(name.name()), "duplicate {}", name.name());
    }
    assert!(IconName::ALL.len() >= 30);
}
