//! Display components: each gallery page of the group shows its demos, and
//! the interactive ones respond.

use nsh_core::prelude::*;
use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_omarchy::{AvatarSize, BadgeVariant, IconName};
use nsh_testing::Headless;

fn gallery() -> Headless {
    Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap()
}

fn open(app: &mut Headless, title: &str) -> Result<(), String> {
    app.wait_for("Omarchy")?;
    app.press(&format!("label:{title}"))?;
    app.wait_for(&format!("text:{title}"))
}

fn expect_texts(app: &mut Headless, texts: &[&str]) -> Result<(), String> {
    for t in texts {
        app.wait_for(&format!("text:{t}"))?;
    }
    Ok(())
}

#[test]
fn panel_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Panel")?;
    expect_texts(
        &mut app,
        &[
            "Workspace",
            "Synced",
            "Release checklist",
            "omarchy-theme-set tokyo-night",
        ],
    )
}

#[test]
fn separator_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Separator")?;
    expect_texts(
        &mut app,
        &["Above the rule", "Below the rule", "Files", "View"],
    )
}

#[test]
fn badge_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Badge")?;
    expect_texts(
        &mut app,
        &[
            "Default", "Accent", "Success", "Warning", "Danger", "Passing",
        ],
    )
}

#[test]
fn keycap_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Keycap")?;
    expect_texts(
        &mut app,
        &["esc", "tab", "super", "space", "Open the launcher"],
    )?;
    assert!(app.screen().count("role:keyboardkey")? >= 5);
    Ok(())
}

#[test]
fn empty_state_action() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Empty state")?;
    expect_texts(&mut app, &["No projects yet", "Inbox zero"])?;
    app.press("label:Create project")?;
    app.wait_for("text:Project created")?;
    assert!(!app.shows("No projects yet"));
    Ok(())
}

#[test]
fn avatar_page_shows_initials() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Avatar")?;
    // Each avatar is labeled with the name; its initials are drawn but hidden
    // from screen readers.
    app.wait_for("label:Grace Hopper")?;
    expect_texts(&mut app, &["AL", "GH", "LT", "HU"])
}

#[test]
fn icon_page_lists_the_set() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Icon")?;
    for name in IconName::ALL {
        app.wait_for(&format!("text:{}", name.name()))?;
    }
    app.wait_for("label:Encrypted")
}

#[test]
fn typography_page() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Typography")?;
    expect_texts(
        &mut app,
        &[
            "Heading",
            "Title",
            "Label",
            "Caption: secondary hints and timestamps",
            "omarchy-update",
        ],
    )
}

#[test]
fn markdown_renders_and_reports_links() -> Result<(), String> {
    let mut app = gallery();
    open(&mut app, "Markdown")?;
    expect_texts(&mut app, &["Release notes", "Highlights", "Press a link"])?;
    app.press("text:links")?;
    app.wait_for("text:Link pressed: https://omarchy.org")
}

#[test]
fn icon_markup_is_well_formed() {
    let mut names = std::collections::HashSet::new();
    for name in IconName::ALL {
        let svg = name.svg_markup();
        assert!(
            svg.starts_with("<svg") && svg.ends_with("</svg>"),
            "{}",
            name.name()
        );
        assert!(svg.contains("currentColor"));
        assert!(
            nsh_core::svg::drawing(&svg).is_ok(),
            "{} does not parse",
            name.name()
        );
        assert!(names.insert(name.name()), "duplicate {}", name.name());
    }
    assert!(IconName::ALL.len() >= 30);
}

/// Every constructor builds in a component of its own.
#[component]
fn Showcase() -> impl IntoElement {
    nsh_omarchy::init();
    view()
        .child(nsh_omarchy::panel("Panel title").child(nsh_omarchy::separator()))
        .child(nsh_omarchy::badge("Beta", BadgeVariant::Accent))
        .child(nsh_omarchy::keycaps(["ctrl", "k"]))
        .child(nsh_omarchy::avatar("Solo", AvatarSize::Small))
        .child(nsh_omarchy::empty_state(
            IconName::Search,
            "No results",
            "Try another query.",
        ))
        .child(nsh_omarchy::title_text("Heading in a test"))
        .child(nsh_omarchy::markdown().source("Some **bold** words"))
}

#[test]
fn constructors_render() -> Result<(), String> {
    let mut app = Headless::start(EngineConfig::new(Root::of::<Showcase>())).unwrap();
    expect_texts(
        &mut app,
        &[
            "Panel title",
            "Beta",
            "ctrl",
            "k",
            "SO",
            "No results",
            "Try another query.",
            "Heading in a test",
            "Some bold words",
        ],
    )
}
