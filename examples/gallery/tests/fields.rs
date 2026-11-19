//! The fields pages, pressed through: typing, stepping, codes, choosing,
//! filtering, picking days and colors. Today is 2026-10-28 (a manual
//! clock).

use std::time::{Duration, SystemTime};

use nsh_engine::{Clock, EngineConfig, Environment, Idiom, Os, Points, Root, Size};
use nsh_testing::Headless;

const GONE: Duration = Duration::from_secs(2);

/// The gallery on page `title`: the first sidebar button of that title
/// (the Forms group comes before any other page of the same name).
fn open_page(title: &str) -> Result<Headless, String> {
    let mut config = EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        environment: Environment {
            os: Os::MacOs,
            idiom: Idiom::Desktop,
            ..Environment::default()
        },
        ..EngineConfig::new(Root::of::<nsh_omarchy_gallery::Gallery>())
    };
    // Noon UTC, 2026-10-28.
    config.clock = Clock::manual(SystemTime::UNIX_EPOCH + Duration::from_secs(1_793_188_800));
    let mut app = Headless::start(config).unwrap();
    app.wait_for("label:Button")?;
    app.press(&format!("label:{title} >> nth=0"))?;
    Ok(app)
}

#[test]
fn input_types_and_submits() -> Result<(), String> {
    let mut app = open_page("Input")?;
    app.wait_for("text:Return submits it.")?;
    app.type_text("label:Workspace name", "dev")?;
    app.wait_for("text:value: \"dev\"  submitted: \"\"")?;
    app.submit("label:Workspace name")?;
    app.wait_for("text:value: \"dev\"  submitted: \"dev\"")?;

    app.type_text("label:Passphrase", "abc")?;
    app.wait_for("text:At least 8 characters.")?;
    app.type_text("label:Passphrase", "defghi")?;
    app.wait_for("text:9 characters")?;
    app.wait_for_gone("text:At least 8 characters.", GONE)?;
    Ok(())
}

#[test]
fn textarea_counts_lines() -> Result<(), String> {
    let mut app = open_page("Textarea")?;
    app.type_text("label:Notes", "one\ntwo")?;
    app.wait_for("text:2 lines, 7 characters")?;
    Ok(())
}

#[test]
fn number_input_steps_within_bounds() -> Result<(), String> {
    let mut app = open_page("Number input")?;
    app.wait_for("text:value: 4")?;
    app.press("label:Increase >> nth=0")?;
    app.wait_for("text:value: 5")?;
    app.type_text("label:Window gaps", "")?;
    app.press_key("pagedown")?;
    app.wait_for("text:value: 0")?;
    Ok(())
}

#[test]
fn otp_input_completes() -> Result<(), String> {
    let mut app = open_page("Otp input")?;
    app.type_text("label:Verification code, 6 digits", "123")?;
    app.wait_for("text:code: \"123\"")?;
    app.type_text("label:Verification code, 6 digits", "456")?;
    app.wait_for("text:complete: 123456")?;
    app.type_text("label:PIN, 4 digits", "0000")?;
    app.wait_for("text:Too easy to guess.")?;
    Ok(())
}

#[test]
fn select_chooses_and_skips_disabled() -> Result<(), String> {
    let mut app = open_page("Select")?;
    app.wait_for("text:value: dwindle")?;
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    assert!(!app.screen().enabled("role:option[Floating]")?);
    app.press("role:option[Master]")?;
    app.wait_for("text:value: master")?;
    app.wait_for_gone("role:listbox", GONE)?;

    app.press("role:combobox[Theme]")?;
    app.wait_for("role:option[Gruvbox]")?;
    app.press("role:option[Gruvbox]")?;
    app.wait_for("text:value: gruvbox")?;
    Ok(())
}

#[test]
fn combobox_filters_and_chooses() -> Result<(), String> {
    let mut app = open_page("Combobox")?;
    app.wait_for("text:value: none")?;
    app.type_text("label:Theme", "gru")?;
    app.wait_for("role:option[Gruvbox]")?;
    assert!(!app.screen().matches("role:option[Nord]")?);
    app.press("role:option[Gruvbox]")?;
    app.wait_for("text:value: gruvbox")?;

    app.type_text("label:Pinned themes", "nord")?;
    app.wait_for("role:option[Nord]")?;
    app.press("role:option[Nord]")?;
    app.wait_for("text:pinned: tokyo-night, nord")?;
    Ok(())
}

#[test]
fn calendar_picks_a_day_and_a_range() -> Result<(), String> {
    let mut app = open_page("Calendar")?;
    app.wait_for("text:picked: none")?;
    app.press("label:2026-10-14 >> nth=0")?;
    app.wait_for("text:picked: 2026-10-14")?;
    app.press("label:2026-10-05 >> nth=1")?;
    app.press("label:2026-10-09 >> nth=1")?;
    app.wait_for("text:range: 2026-10-05 to 2026-10-09")?;
    // After today nothing can be picked in the range.
    assert!(!app.screen().enabled("label:2026-10-30 >> nth=1")?);
    Ok(())
}

#[test]
fn date_picker_opens_picks_and_closes() -> Result<(), String> {
    let mut app = open_page("Date picker")?;
    app.wait_for("text:Choose a date")?;
    app.wait_for("text:Choose dates")?;
    app.wait_for("text:Nov 19, 2026")?;
    app.press("role:combobox[Due date]")?;
    app.wait_for("label:2026-10-21")?;
    app.press("label:2026-10-21")?;
    app.wait_for("text:due: 2026-10-21")?;
    app.wait_for("text:Oct 21, 2026")?;
    app.wait_for_gone("label:2026-10-21", GONE)?;

    app.press("role:combobox[Trip dates]")?;
    app.wait_for("label:2026-10-29")?;
    app.press("label:2026-10-29")?;
    app.press("label:2026-10-31")?;
    app.wait_for("text:range: 2026-10-29 to 2026-10-31")?;
    app.wait_for("text:Oct 29, 2026 – Oct 31, 2026")?;
    Ok(())
}

#[test]
fn color_picker_takes_a_typed_color() -> Result<(), String> {
    let mut app = open_page("Color picker")?;
    app.wait_for("label:Hue")?;
    app.wait_for("label:Opacity")?;
    app.clear("label:Hex color")?;
    app.type_text("label:Hex color", "#ff0000")?;
    app.submit("label:Hex color")?;
    app.wait_for("text:color: #FF0000")?;
    Ok(())
}
