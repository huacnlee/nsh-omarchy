//! The `fields` pages of the gallery, driven headless: typing into a text
//! field, stepping a number, entering a code, choosing a select's option
//! and filtering a combobox. Each page shows its value as `value: …`.

use nsh_engine::{EngineConfig, Points, Root, Size};
use nsh_testing::Headless;

fn open(page: &str) -> Headless {
    let mut app = Headless::start(EngineConfig {
        viewport: Size::new(Points(1200.0), Points(800.0)),
        ..EngineConfig::new(Root::of::<nsh_omarchy::gallery::Gallery>())
    })
    .unwrap();
    app.wait_for("Omarchy").unwrap();
    app.press(&format!("label:{page}")).unwrap();
    app
}

#[test]
fn typing_into_an_input_updates_its_value() -> Result<(), String> {
    let mut app = open("Input");
    app.wait_for("text:value: \"\"")?;
    app.type_text("label:Workspace name", "dev")?;
    app.wait_for("text:value: \"dev\"")?;
    assert_eq!(
        app.screen().input_text("label:Workspace name")?.as_deref(),
        Some("dev")
    );
    app.type_text("label:Passphrase", "hunter2")?;
    app.wait_for("text:7 characters")?;
    assert!(!app.screen().enabled("label:Config path")?);
    Ok(())
}

#[test]
fn typing_into_a_textarea_counts_lines() -> Result<(), String> {
    let mut app = open("Textarea");
    app.type_text("label:Notes", "one\ntwo")?;
    app.wait_for("text:2 lines, 7 characters")?;
    Ok(())
}

#[test]
fn a_number_field_steps_by_keys_and_stops_at_its_bounds() -> Result<(), String> {
    let mut app = open("Number input");
    app.wait_for("text:value: 4")?;
    // Focus the field without changing it, then step.
    app.type_text("label:Window gaps", "")?;
    app.press_key("up")?;
    app.wait_for("text:value: 5")?;
    app.press_key("down")?;
    app.press_key("down")?;
    app.wait_for("text:value: 3")?;
    app.press_key("pageup")?;
    app.wait_for("text:value: 13")?;
    app.press_key("pageup")?;
    app.wait_for("text:value: 20")?;
    Ok(())
}

#[test]
fn a_number_field_steps_by_its_buttons() -> Result<(), String> {
    let mut app = open("Number input");
    app.wait_for("text:value: 4")?;
    app.press("text:+ >> nth=0")?;
    app.wait_for("text:value: 5")?;
    app.press("text:\u{2212} >> nth=0")?;
    app.press("text:\u{2212} >> nth=0")?;
    app.wait_for("text:value: 3")?;
    Ok(())
}

#[test]
fn an_otp_field_takes_digits_and_completes() -> Result<(), String> {
    let mut app = open("Otp input");
    app.type_text("label:Verification code, 6 digits", "12a3")?;
    app.wait_for("text:code: \"123\"")?;
    app.type_text("label:Verification code, 6 digits", "456")?;
    app.wait_for("text:complete: 123456")?;
    Ok(())
}

#[test]
fn choosing_a_select_option_by_press_and_by_keys() -> Result<(), String> {
    let mut app = open("Select");
    app.wait_for("text:value: dwindle")?;
    app.wait_for("text:Dwindle")?;
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    app.press("role:option[Master]")?;
    app.wait_for("text:value: master")?;
    // The trigger shows the chosen title.
    app.wait_for("text:Master")?;
    app.wait_for_gone("role:listbox", std::time::Duration::from_secs(2))?;

    // Keys: the list opens on the chosen row, Down skips the disabled
    // Floating, Return chooses.
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    app.press_key("down")?;
    app.press_key("down")?;
    app.press_key("enter")?;
    app.wait_for("text:value: monocle")?;
    Ok(())
}

#[test]
fn a_disabled_option_cannot_be_chosen() -> Result<(), String> {
    let mut app = open("Select");
    app.press("role:combobox[Layout]")?;
    app.wait_for("role:listbox")?;
    assert!(!app.screen().enabled("role:option[Floating]")?);
    Ok(())
}

#[test]
fn a_combobox_filters_its_options_as_it_is_typed_into() -> Result<(), String> {
    let mut app = open("Combobox");
    app.wait_for("text:value: none  query: \"\"")?;
    app.type_text("label:Theme", "nor")?;
    app.wait_for("role:option[Nord]")?;
    app.wait_for("text:value: none  query: \"nor\"")?;
    assert_eq!(app.screen().count("role:option")?, 1);
    assert!(!app.screen().matches("role:option[Tokyo Night]")?);
    app.press("role:option[Nord]")?;
    app.wait_for("text:/^value: nord /")?;

    // Nothing matches: the empty note shows.
    app.clear("label:Theme")?;
    app.type_text("label:Theme", "zzz")?;
    app.wait_for("text:No matching options")?;
    assert_eq!(app.screen().count("role:option")?, 0);
    Ok(())
}
